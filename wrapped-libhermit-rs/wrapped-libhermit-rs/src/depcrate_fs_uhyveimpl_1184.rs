// Generated macro for impl_1184 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1184 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1184"}
// Dependencies: {}
impl UhyveDirectory { pub const fn new (prefix : Option < String >) -> Self { UhyveDirectory { prefix } } fn traversal_path (& self , components : & [& str]) -> CString { let prefix_deref = self . prefix . as_deref () ; let components_with_prefix = prefix_deref . iter () . chain (components . iter () . rev ()) ; let path : String = components_with_prefix . flat_map (| component | ["/" , component]) . skip (1) . collect () ; if path . is_empty () { CString :: new ("/") . unwrap () } else { CString :: new (path) . unwrap () } } }
};
}
