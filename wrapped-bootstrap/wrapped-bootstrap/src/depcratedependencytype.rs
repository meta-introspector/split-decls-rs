// Generated macro for DependencyType (enum)
macro_rules! DepcrateDependencyType {
() => {
// Module: crate
// Provides: {"DependencyType"}
// Dependencies: {}
# [doc = " When building Rust various objects are handled differently."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub enum DependencyType { # [doc = " Libraries originating from proc-macros."] Host , # [doc = " Typical Rust libraries."] Target , # [doc = " Non Rust libraries and objects shipped to ease usage of certain targets."] TargetSelfContained , }
};
}
