// Generated macro for impl_8261 (impl)
macro_rules! Depcrate_non_std_lazy_staticsimpl_8261 {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"impl_8261"}
// Dependencies: {}
impl NonStdLazyStatic { # [must_use] pub fn new (conf : & 'static Conf) -> Self { Self { msrv : conf . msrv , once_cell_crates : Vec :: new () , sugg_map : FxIndexMap :: default () , lazy_type_defs : FxIndexMap :: default () , uses_other_once_cell_types : false , } } }
};
}
