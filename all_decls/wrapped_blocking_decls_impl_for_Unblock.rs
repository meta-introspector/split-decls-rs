use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Unblock<T> {
    /// Wraps a blocking I/O handle into the async [`Unblock`] interface.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use blocking::Unblock;
    ///
    /// let stdin = Unblock::new(std::io::stdin());
    /// ```
    pub fn new(io: T) -> Unblock<T> {
        Unblock {
            state: State::Idle(Some(Box::new(io))),
            cap: None,
        }
    }
    /// Wraps a blocking I/O handle into the async [`Unblock`] interface with a custom buffer
    /// capacity.
    ///
    /// When communicating with the inner [`Stream`]/[`Read`]/[`Write`] type from async code, data
    /// transferred between blocking and async code goes through a buffer of limited capacity. This
    /// constructor configures that capacity.
    ///
    /// The default capacity is:
    ///
    /// * For [`Iterator`] types: 8192 items.
    /// * For [`Read`]/[`Write`] types: 8 MB.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use blocking::Unblock;
    ///
    /// let stdout = Unblock::with_capacity(64 * 1024, std::io::stdout());
    /// ```
    pub fn with_capacity(cap: usize, io: T) -> Unblock<T> {
        Unblock {
            state: State::Idle(Some(Box::new(io))),
            cap: Some(cap),
        }
    }
    /// Gets a mutable reference to the blocking I/O handle.
    ///
    /// This is an async method because the I/O handle might be on the thread pool and needs to
    /// be moved onto the current thread before we can get a reference to it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use blocking::{unblock, Unblock};
    /// use std::fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = unblock(|| File::create("file.txt")).await?;
    /// let mut file = Unblock::new(file);
    ///
    /// let metadata = file.get_mut().await.metadata()?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn get_mut(&mut self) -> &mut T {
        future::poll_fn(|cx| self.poll_stop(cx)).await.ok();
        match &mut self.state {
            State::Idle(t) => t.as_mut().expect("inner value was taken out"),
            State::WithMut(..)
            | State::Streaming(..)
            | State::Reading(..)
            | State::Writing(..)
            | State::Seeking(..) => {
                unreachable!("when stopped, the state machine must be in idle state");
            }
        }
    }
    /// Performs a blocking operation on the I/O handle.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use blocking::{unblock, Unblock};
    /// use std::fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = unblock(|| File::create("file.txt")).await?;
    /// let mut file = Unblock::new(file);
    ///
    /// let metadata = file.with_mut(|f| f.metadata()).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn with_mut<R, F>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut T) -> R + Send + 'static,
        R: Send + 'static,
        T: Send + 'static,
    {
        future::poll_fn(|cx| self.poll_stop(cx)).await.ok();
        let mut t = match &mut self.state {
            State::Idle(t) => t.take().expect("inner value was taken out"),
            State::WithMut(..)
            | State::Streaming(..)
            | State::Reading(..)
            | State::Writing(..)
            | State::Seeking(..) => {
                unreachable!("when stopped, the state machine must be in idle state");
            }
        };
        let (sender, receiver) = bounded(1);
        let task = Executor::spawn(async move {
            sender.try_send(op(&mut t)).ok();
            t
        });
        self.state = State::WithMut(task);
        receiver.recv().await.expect("`Unblock::with_mut()` operation has panicked")
    }
    /// Extracts the inner blocking I/O handle.
    ///
    /// This is an async method because the I/O handle might be on the thread pool and needs to
    /// be moved onto the current thread before we can extract it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use blocking::{unblock, Unblock};
    /// use futures_lite::prelude::*;
    /// use std::fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = unblock(|| File::create("file.txt")).await?;
    /// let file = Unblock::new(file);
    ///
    /// let file = file.into_inner().await;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn into_inner(self) -> T {
        let mut this = self;
        future::poll_fn(|cx| this.poll_stop(cx)).await.ok();
        match &mut this.state {
            State::Idle(t) => *t.take().expect("inner value was taken out"),
            State::WithMut(..)
            | State::Streaming(..)
            | State::Reading(..)
            | State::Writing(..)
            | State::Seeking(..) => {
                unreachable!("when stopped, the state machine must be in idle state");
            }
        }
    }
    /// Waits for the running task to stop.
    ///
    /// On success, the state machine is moved into the idle state.
    fn poll_stop(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        loop {
            match &mut self.state {
                State::Idle(_) => return Poll::Ready(Ok(())),
                State::WithMut(task) => {
                    let io = ready!(Pin::new(task).poll(cx));
                    self.state = State::Idle(Some(io));
                }
                State::Streaming(any, task) => {
                    any.take();
                    let iter = ready!(Pin::new(task).poll(cx));
                    self.state = State::Idle(Some(iter));
                }
                State::Reading(reader, task) => {
                    reader.take();
                    let (res, io) = ready!(Pin::new(task).poll(cx));
                    self.state = State::Idle(Some(io));
                    res?;
                }
                State::Writing(writer, task) => {
                    writer.take();
                    let (res, io) = ready!(Pin::new(task).poll(cx));
                    self.state = State::Idle(Some(io));
                    res?;
                }
                State::Seeking(task) => {
                    let (_, res, io) = ready!(Pin::new(task).poll(cx));
                    self.state = State::Idle(Some(io));
                    res?;
                }
            }
        }
    }
}
