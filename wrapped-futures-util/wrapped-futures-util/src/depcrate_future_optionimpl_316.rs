// Generated macro for impl_316 (impl)
macro_rules! Depcrate_future_optionimpl_316 {
() => {
// Module: crate::future::option
// Provides: {"impl_316"}
// Dependencies: {}
impl < F : FusedFuture > FusedFuture for OptionFuture < F > { fn is_terminated (& self) -> bool { match & self . inner { Some (x) => x . is_terminated () , None => true , } } }
};
}
