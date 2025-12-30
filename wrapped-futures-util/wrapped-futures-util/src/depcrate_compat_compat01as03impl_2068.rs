// Generated macro for impl_2068 (impl)
macro_rules! Depcrate_compat_compat01as03impl_2068 {
() => {
// Module: crate::compat::compat01as03
// Provides: {"impl_2068"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , SinkItem > Compat01As03Sink < S , SinkItem > { # [doc = " Wraps a futures 0.1 Sink object in a futures 0.3-compatible wrapper."] pub fn new (inner : S) -> Self { Self { inner : spawn01 (inner) , buffer : None , close_started : false } } fn in_notify < R > (& mut self , cx : & mut Context < '_ > , f : impl FnOnce (& mut S) -> R) -> R { let notify = & WakerToHandle (cx . waker ()) ; self . inner . poll_fn_notify (notify , 0 , f) } # [doc = " Get a reference to 0.1 Sink object contained within."] pub fn get_ref (& self) -> & S { self . inner . get_ref () } # [doc = " Get a mutable reference to 0.1 Sink contained within."] pub fn get_mut (& mut self) -> & mut S { self . inner . get_mut () } # [doc = " Consume this wrapper to return the underlying 0.1 Sink."] pub fn into_inner (self) -> S { self . inner . into_inner () } }
};
}
