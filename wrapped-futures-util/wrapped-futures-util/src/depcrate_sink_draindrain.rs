// Generated macro for drain (function)
macro_rules! Depcrate_sink_draindrain {
() => {
// Module: crate::sink::drain
// Provides: {"drain"}
// Dependencies: {}
# [doc = " Create a sink that will just discard all items given to it."] # [doc = ""] # [doc = " Similar to [`io::Sink`](::std::io::Sink)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::sink::{self, SinkExt};"] # [doc = ""] # [doc = " let mut drain = sink::drain();"] # [doc = " drain.send(5).await?;"] # [doc = " # Ok::<(), std::convert::Infallible>(()) }).unwrap();"] # [doc = " ```"] pub fn drain < T > () -> Drain < T > { assert_sink :: < T , Infallible , _ > (Drain { marker : PhantomData }) }
};
}
