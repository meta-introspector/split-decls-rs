// Generated macro for impl_11445 (impl)
macro_rules! Depcrate_wildcard_importsimpl_11445 {
() => {
// Module: crate::wildcard_imports
// Provides: {"impl_11445"}
// Dependencies: {}
impl WildcardImports { fn check_exceptions (& self , cx : & LateContext < '_ > , item : & Item < '_ > , segments : & [PathSegment < '_ >]) -> bool { item . span . from_expansion () || is_prelude_import (segments) || is_allowed_via_config (segments , & self . allowed_segments) || (is_super_only_import (segments) && is_in_test (cx . tcx , item . hir_id ())) } }
};
}
