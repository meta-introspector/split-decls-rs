// Generated macro for impl_362 (impl)
macro_rules! Depcrate_expand_autodiff_attrsimpl_362 {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"impl_362"}
// Dependencies: {}
impl AutoDiffAttrs { pub fn has_primal_ret (& self) -> bool { matches ! (self . ret_activity , DiffActivity :: Active | DiffActivity :: Dual) } }
};
}
