// Generated macro for impl_669 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_669 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_669"}
// Dependencies: {}
impl Step for Distcheck { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("distcheck") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Distcheck) ; } # [doc = " Runs `distcheck`, which is a collection of smoke tests:"] # [doc = ""] # [doc = " - Run `make check` from an unpacked dist tarball to make sure we can at the minimum run"] # [doc = "   check steps from those sources."] # [doc = " - Check that selected dist components (`rust-src` only at the moment) at least have expected"] # [doc = "   directory shape and crate manifests that cargo can generate a lockfile from."] # [doc = " - Check that we can run `cargo metadata` on the workspace in the `rustc-dev` component"] # [doc = ""] # [doc = " FIXME(#136822): dist components are under-tested."] fn run (self , builder : & Builder < '_ >) { let root_dir = std :: env :: temp_dir () . join ("distcheck") ; distcheck_plain_source_tarball (builder , & root_dir . join ("distcheck-rustc-src")) ; distcheck_rust_src (builder , & root_dir . join ("distcheck-rust-src")) ; distcheck_rustc_dev (builder , & root_dir . join ("distcheck-rustc-dev")) ; } }
};
}
