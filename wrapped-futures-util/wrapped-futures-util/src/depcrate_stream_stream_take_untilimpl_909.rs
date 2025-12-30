// Generated macro for impl_909 (impl)
macro_rules! Depcrate_stream_stream_take_untilimpl_909 {
() => {
// Module: crate::stream::stream::take_until
// Provides: {"impl_909"}
// Dependencies: {}
impl < St , Fut > fmt :: Debug for TakeUntil < St , Fut > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : Future + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TakeUntil") . field ("stream" , & self . stream) . field ("fut" , & self . fut) . finish () } }
};
}
