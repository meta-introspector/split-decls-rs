// Generated macro for distcheck_rustc_dev (function)
macro_rules! Depcrate_core_build_steps_testdistcheck_rustc_dev {
() => {
// Module: crate::core::build_steps::test
// Provides: {"distcheck_rustc_dev"}
// Dependencies: {}
# [doc = " Check that rustc-dev's compiler crate source code can be loaded with `cargo metadata`"] fn distcheck_rustc_dev (builder : & Builder < '_ > , dir : & Path) { builder . info ("Distcheck rustc-dev") ; let tarball = builder . ensure (dist :: RustcDev :: new (builder , builder . host_target)) . unwrap () ; builder . clear_dir (dir) ; command ("tar") . arg ("-xf") . arg (tarball . tarball ()) . arg ("--strip-components=1") . current_dir (dir) . run (builder) ; command (& builder . initial_cargo) . arg ("metadata") . arg ("--manifest-path") . arg ("rustc-dev/lib/rustlib/rustc-src/rust/compiler/rustc/Cargo.toml") . env ("RUSTC_BOOTSTRAP" , "1") . env ("RUSTC" , & builder . initial_rustc) . current_dir (dir) . run (builder) ; builder . remove_dir (dir) ; }
};
}
