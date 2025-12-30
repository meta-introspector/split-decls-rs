// Generated macro for distcheck_rust_src (function)
macro_rules! Depcrate_core_build_steps_testdistcheck_rust_src {
() => {
// Module: crate::core::build_steps::test
// Provides: {"distcheck_rust_src"}
// Dependencies: {}
# [doc = " Check that rust-src has all of libstd's dependencies"] fn distcheck_rust_src (builder : & Builder < '_ > , src_dir : & Path) { builder . info ("Distcheck rust-src") ; let src_tarball = builder . ensure (dist :: Src) ; builder . clear_dir (src_dir) ; command ("tar") . arg ("-xf") . arg (src_tarball . tarball ()) . arg ("--strip-components=1") . current_dir (src_dir) . run (builder) ; let toml = src_dir . join ("rust-src/lib/rustlib/src/rust/library/std/Cargo.toml") ; command (& builder . initial_cargo) . env ("RUSTC_BOOTSTRAP" , "1") . arg ("generate-lockfile") . arg ("--manifest-path") . arg (& toml) . current_dir (src_dir) . run (builder) ; builder . remove_dir (src_dir) ; }
};
}
