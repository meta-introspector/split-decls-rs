// Generated macro for impl_359 (impl)
macro_rules! Depcrate_expand_autodiff_attrsimpl_359 {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"impl_359"}
// Dependencies: {}
impl DiffActivity { pub fn is_dual_or_const (& self) -> bool { use DiffActivity :: * ; matches ! (self , | Dual | DualOnly | Dualv | DualvOnly | Const) } }
};
}
