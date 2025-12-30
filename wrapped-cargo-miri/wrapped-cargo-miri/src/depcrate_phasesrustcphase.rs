// Generated macro for RustcPhase (enum)
macro_rules! Depcrate_phasesRustcPhase {
() => {
// Module: crate::phases
// Provides: {"RustcPhase"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq)] pub enum RustcPhase { # [doc = " `rustc` called during sysroot build."] Setup , # [doc = " `rustc` called by `cargo` for regular build."] Build , # [doc = " `rustc` called by `rustdoc` for doctest."] Rustdoc , }
};
}
