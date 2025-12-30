// Generated macro for impl_21 (impl)
macro_rules! Depcrate_rewrites_trackerimpl_21 {
() => {
// Module: crate::rewrites::tracker
// Provides: {"impl_21"}
// Dependencies: {}
# [doc = " Lifecycle"] impl < T : Change > Tracker < T > { # [doc = " Create a new instance with `rewrites` configuration."] pub fn new (rewrites : Rewrites) -> Self { Tracker { items : vec ! [] , path_backing : vec ! [] , rewrites , child_renames : Default :: default () , } } }
};
}
