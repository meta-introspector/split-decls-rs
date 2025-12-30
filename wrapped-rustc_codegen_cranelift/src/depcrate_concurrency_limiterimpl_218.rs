// Generated macro for impl_218 (impl)
macro_rules! Depcrate_concurrency_limiterimpl_218 {
() => {
// Module: crate::concurrency_limiter
// Provides: {"impl_218"}
// Dependencies: {}
impl Drop for ConcurrencyLimiter { fn drop (& mut self) { if ! self . finished && ! std :: thread :: panicking () { panic ! ("Forgot to call finished() on ConcurrencyLimiter") ; } } }
};
}
