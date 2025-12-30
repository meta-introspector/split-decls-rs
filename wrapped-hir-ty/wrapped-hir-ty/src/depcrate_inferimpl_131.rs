// Generated macro for impl_131 (impl)
macro_rules! Depcrate_inferimpl_131 {
() => {
// Module: crate::infer
// Provides: {"impl_131"}
// Dependencies: {}
impl < T > InferOk < T > { fn map < U > (self , f : impl FnOnce (T) -> U) -> InferOk < U > { InferOk { value : f (self . value) , goals : self . goals } } }
};
}
