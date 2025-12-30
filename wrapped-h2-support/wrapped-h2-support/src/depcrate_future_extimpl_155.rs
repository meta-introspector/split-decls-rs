// Generated macro for impl_155 (impl)
macro_rules! Depcrate_future_extimpl_155 {
() => {
// Module: crate::future_ext
// Provides: {"impl_155"}
// Dependencies: {}
# [doc = " A future that only polls the inner future if it has been woken (after the initial poll)."] impl < T > Future for Wakened < T > where T : Future , { type Output = T :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . get_mut () ; if ! this . woken . load (std :: sync :: atomic :: Ordering :: SeqCst) { return Poll :: Pending ; } this . woken . store (false , std :: sync :: atomic :: Ordering :: SeqCst) ; let my_waker = IfWokenWaker { inner : cx . waker () . clone () , wakened : this . woken . clone () , } ; let my_waker = Arc :: new (my_waker) . into () ; let mut cx = Context :: from_waker (& my_waker) ; this . future . as_mut () . poll (& mut cx) } }
};
}
