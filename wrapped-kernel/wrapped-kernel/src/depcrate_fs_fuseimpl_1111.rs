// Generated macro for impl_1111 (impl)
macro_rules! Depcrate_fs_fuseimpl_1111 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1111"}
// Dependencies: {}
impl FuseDirectory { pub fn new (prefix : Option < String >) -> Self { let microseconds = arch :: kernel :: systemtime :: now_micros () ; let t = timespec :: from_usec (microseconds as i64) ; FuseDirectory { prefix , attr : FileAttr { st_mode : AccessPermission :: from_bits (0o777) . unwrap () | AccessPermission :: S_IFDIR , st_atim : t , st_mtim : t , st_ctim : t , .. Default :: default () } , } } fn traversal_path (& self , components : & [& str]) -> CString { let prefix_deref = self . prefix . as_deref () ; let components_with_prefix = prefix_deref . iter () . chain (components . iter () . rev ()) ; let path : String = components_with_prefix . flat_map (| component | ["/" , component]) . collect () ; if path . is_empty () { CString :: new ("/") . unwrap () } else { CString :: new (path) . unwrap () } } }
};
}
