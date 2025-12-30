// Generated macro for pkgname (function)
macro_rules! Depcrate_core_build_steps_distpkgname {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"pkgname"}
// Dependencies: {}
pub fn pkgname (builder : & Builder < '_ > , component : & str) -> String { format ! ("{}-{}" , component , builder . rust_package_vers ()) }
};
}
