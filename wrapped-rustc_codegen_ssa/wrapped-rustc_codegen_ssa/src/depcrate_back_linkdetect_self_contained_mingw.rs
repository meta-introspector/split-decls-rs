// Generated macro for detect_self_contained_mingw (function)
macro_rules! Depcrate_back_linkdetect_self_contained_mingw {
() => {
// Module: crate::back::link
// Provides: {"detect_self_contained_mingw"}
// Dependencies: {}
fn detect_self_contained_mingw (sess : & Session , linker : & Path) -> bool { if linker == Path :: new ("rust-lld") { return true ; } let linker_with_extension = if cfg ! (windows) && linker . extension () . is_none () { linker . with_extension ("exe") } else { linker . to_path_buf () } ; for dir in env :: split_paths (& env :: var_os ("PATH") . unwrap_or_default ()) { let full_path = dir . join (& linker_with_extension) ; if full_path . is_file () && ! full_path . starts_with (sess . opts . sysroot . path ()) { return false ; } } true }
};
}
