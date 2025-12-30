// Generated macro for Kind (enum)
macro_rules! Depcrate_core_builderKind {
() => {
// Module: crate::core::builder
// Provides: {"Kind"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Eq , Hash , PartialEq , PartialOrd , Ord , ValueEnum)] pub enum Kind { # [value (alias = "b")] Build , # [value (alias = "c")] Check , Clippy , Fix , Format , # [value (alias = "t")] Test , Miri , MiriSetup , MiriTest , Bench , # [value (alias = "d")] Doc , Clean , Dist , Install , # [value (alias = "r")] Run , Setup , Vendor , Perf , }
};
}
