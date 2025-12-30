// Generated macro for rehome_sysroot_lib_dir (function)
macro_rules! Depcrate_back_linkrehome_sysroot_lib_dir {
() => {
// Module: crate::back::link
// Provides: {"rehome_sysroot_lib_dir"}
// Dependencies: {}
fn rehome_sysroot_lib_dir (sess : & Session , lib_dir : & Path) -> PathBuf { let sysroot_lib_path = & sess . target_tlib_path . dir ; let canonical_sysroot_lib_path = { try_canonicalize (sysroot_lib_path) . unwrap_or_else (| _ | sysroot_lib_path . clone ()) } ; let canonical_lib_dir = try_canonicalize (lib_dir) . unwrap_or_else (| _ | lib_dir . to_path_buf ()) ; if canonical_lib_dir == canonical_sysroot_lib_path { sysroot_lib_path . clone () } else { fix_windows_verbatim_for_gcc (lib_dir) } }
};
}
