// Generated macro for impl_11043 (impl)
macro_rules! Depcrate_wildcard_importsimpl_11043 {
() => {
// Module: crate::wildcard_imports
// Provides: {"impl_11043"}
// Dependencies: {}
impl WildcardImports { fn check_exceptions (& self , cx : & LateContext < '_ > , item : & Item < '_ > , segments : & [PathSegment < '_ >]) -> bool { item . span . from_expansion () || is_prelude_import (segments) || is_allowed_via_config (segments , & self . allowed_segments) || (is_super_only_import (segments) && is_in_test (cx . tcx , item . hir_id ())) } }
};
}
