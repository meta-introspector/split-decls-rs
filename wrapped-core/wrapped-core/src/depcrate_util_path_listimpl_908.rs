// Generated macro for impl_908 (impl)
macro_rules! Depcrate_util_path_listimpl_908 {
() => {
// Module: crate::util::path_list
// Provides: {"impl_908"}
// Dependencies: {}
impl FromMeta for PathList { fn from_list (v : & [NestedMeta]) -> Result < Self > { let mut paths = Vec :: with_capacity (v . len ()) ; for nmi in v { if let NestedMeta :: Meta (Meta :: Path (ref path)) = * nmi { paths . push (path . clone ()) ; } else { return Err (Error :: unexpected_type ("non-word") . with_span (nmi)) ; } } Ok (PathList (paths)) } }
};
}
