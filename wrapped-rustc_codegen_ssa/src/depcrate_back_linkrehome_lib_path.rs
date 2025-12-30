// Generated macro for rehome_lib_path (function)
macro_rules! Depcrate_back_linkrehome_lib_path {
() => {
// Module: crate::back::link
// Provides: {"rehome_lib_path"}
// Dependencies: {}
fn rehome_lib_path (sess : & Session , path : & Path) -> PathBuf { if let Some (dir) = path . parent () { let file_name = path . file_name () . expect ("library path has no file name component") ; rehome_sysroot_lib_dir (sess , dir) . join (file_name) } else { fix_windows_verbatim_for_gcc (path) } }
};
}
