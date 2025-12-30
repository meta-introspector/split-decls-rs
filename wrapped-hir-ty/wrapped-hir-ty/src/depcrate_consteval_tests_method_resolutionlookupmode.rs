// Generated macro for LookupMode (enum)
macro_rules! Depcrate_consteval_tests_method_resolutionLookupMode {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"LookupMode"}
// Dependencies: {}
# [doc = " Whether we're looking up a dotted method call (like `v.len()`) or a path"] # [doc = " (like `Vec::new`)."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum LookupMode { # [doc = " Looking up a method call like `v.len()`: We only consider candidates"] # [doc = " that have a `self` parameter, and do autoderef."] MethodCall , # [doc = " Looking up a path like `Vec::new` or `Vec::default`: We consider all"] # [doc = " candidates including associated constants, but don't do autoderef."] Path , }
};
}
