// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_writebackimpl_1062 {
() => {
// Module: crate::writeback
// Provides: {"impl_1062"}
// Dependencies: {}
impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for EagerlyNormalizeConsts < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { self . tcx . try_normalize_erasing_regions (self . typing_env , ct) . unwrap_or (ct) } }
};
}
