// Generated macro for impl_156 (impl)
macro_rules! Depcrate_future_extimpl_156 {
() => {
// Module: crate::future_ext
// Provides: {"impl_156"}
// Dependencies: {}
impl Wake for IfWokenWaker { fn wake (self : Arc < Self >) { self . wakened . store (true , std :: sync :: atomic :: Ordering :: SeqCst) ; self . inner . wake_by_ref () ; } }
};
}
