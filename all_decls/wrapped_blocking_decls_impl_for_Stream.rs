use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Iterator + Send + 'static> Stream for Unblock<T>
where
    T::Item: Send + 'static,
{
    type Item = T::Item;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<T::Item>> {
        loop {
            match &mut self.state {
                State::WithMut(..)
                | State::Streaming(None, _)
                | State::Reading(..)
                | State::Writing(..)
                | State::Seeking(..) => {
                    ready!(self.poll_stop(cx)).ok();
                }
                State::Idle(iter) => {
                    let mut iter = iter.take().expect("inner iterator was taken out");
                    let (sender, receiver) = bounded(self.cap.unwrap_or(8 * 1024));
                    let task = Executor::spawn(async move {
                        for item in &mut iter {
                            if sender.send(item).await.is_err() {
                                break;
                            }
                        }
                        iter
                    });
                    self.state = State::Streaming(
                        Some(Box::new(Box::pin(receiver))),
                        task,
                    );
                }
                State::Streaming(Some(any), task) => {
                    let receiver = any
                        .downcast_mut::<Pin<Box<Receiver<T::Item>>>>()
                        .unwrap();
                    let opt = ready!(receiver.as_mut().poll_next(cx));
                    if opt.is_none() {
                        let iter = ready!(Pin::new(task).poll(cx));
                        self.state = State::Idle(Some(iter));
                    }
                    return Poll::Ready(opt);
                }
            }
        }
    }
}
