// Generated macro for AttrChecker (struct)
macro_rules! Depcrate_significant_drop_tighteningAttrChecker {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"AttrChecker"}
// Dependencies: {}
# [doc = " Checks the existence of the `#[has_significant_drop]` attribute."] struct AttrChecker < 'cx , 'others , 'tcx > { cx : & 'cx LateContext < 'tcx > , type_cache : & 'others mut FxHashMap < Ty < 'tcx > , bool > , }
};
}
