// Generated macro for impl_265 (impl)
macro_rules! Depcrate_future_lazyimpl_265 {
() => {
// Module: crate::future::lazy
// Provides: {"impl_265"}
// Dependencies: {}
impl < F , R > Future for Lazy < F > where F : FnOnce (& mut Context < '_ >) -> R , { type Output = R ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < R > { Poll :: Ready ((self . f . take () . expect ("Lazy polled after completion")) (cx)) } }
};
}
