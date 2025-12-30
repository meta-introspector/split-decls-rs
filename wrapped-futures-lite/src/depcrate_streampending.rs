// Generated macro for pending (function)
macro_rules! Depcrate_streampending {
() => {
// Module: crate::stream
// Provides: {"pending"}
// Dependencies: {}
# [doc = " Creates a stream that is always pending."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut s = stream::pending::<i32>();"] # [doc = " s.next().await;"] # [doc = " unreachable!();"] # [doc = " # })"] # [doc = " ```"] pub fn pending < T > () -> Pending < T > { Pending { _marker : PhantomData , } }
};
}
