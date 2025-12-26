use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Read + Send + 'static> AsyncRead for Unblock<T> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            match &mut self.state {
                State::WithMut(..)
                | State::Reading(None, _)
                | State::Streaming(..)
                | State::Writing(..)
                | State::Seeking(..) => {
                    ready!(self.poll_stop(cx))?;
                }
                State::Idle(io) => {
                    let mut io = io.take().expect("inner value was taken out");
                    let (reader, mut writer) = pipe(self.cap.unwrap_or(8 * 1024 * 1024));
                    let task = Executor::spawn(async move {
                        loop {
                            match future::poll_fn(|cx| writer.poll_fill(cx, &mut io))
                                .await
                            {
                                Ok(0) => return (Ok(()), io),
                                Ok(_) => {}
                                Err(err) => return (Err(err), io),
                            }
                        }
                    });
                    self.state = State::Reading(Some(reader), task);
                }
                State::Reading(Some(reader), task) => {
                    let n = ready!(reader.poll_drain(cx, buf))?;
                    if n == 0 {
                        let (res, io) = ready!(Pin::new(task).poll(cx));
                        self.state = State::Idle(Some(io));
                        res?;
                    }
                    return Poll::Ready(Ok(n));
                }
            }
        }
    }
}
