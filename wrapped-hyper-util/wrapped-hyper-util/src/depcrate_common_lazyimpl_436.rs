// Generated macro for impl_436 (impl)
macro_rules! Depcrate_common_lazyimpl_436 {
() => {
// Module: crate::common::lazy
// Provides: {"impl_436"}
// Dependencies: {}
impl < F , R > Started for Lazy < F , R > where F : FnOnce () -> R , R : Future , { fn started (& self) -> bool { match self . inner { Inner :: Init { .. } => false , Inner :: Fut { .. } | Inner :: Empty => true , } } }
};
}
