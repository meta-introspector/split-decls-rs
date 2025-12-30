// Generated macro for get_rustc_src (function)
macro_rules! Depcrate_sysrootget_rustc_src {
() => {
// Module: crate::sysroot
// Provides: {"get_rustc_src"}
// Dependencies: {}
fn get_rustc_src (sysroot_path : & AbsPath) -> Option < ManifestPath > { let rustc_src = sysroot_path . join ("lib/rustlib/rustc-src/rust/compiler/rustc/Cargo.toml") ; let rustc_src = ManifestPath :: try_from (rustc_src) . ok () ? ; tracing :: debug ! ("checking for rustc source code: {rustc_src}") ; if fs :: metadata (& rustc_src) . is_ok () { Some (rustc_src) } else { None } }
};
}
