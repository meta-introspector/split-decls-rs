use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "futures-01")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures-01")))]
impl<T: futures_01::Sink> futures_01::Sink for Instrumented<T> {
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;
    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> futures_01::StartSend<Self::SinkItem, Self::SinkError> {
        let _enter = self.span.enter();
        self.inner.start_send(item)
    }
    fn poll_complete(&mut self) -> futures_01::Poll<(), Self::SinkError> {
        let _enter = self.span.enter();
        self.inner.poll_complete()
    }
}
