// Generated macro for tests (module)
macro_rules! Depcrate_nfa_thompson_buildertests {
() => {
// Module: crate::nfa::thompson::builder
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn state_has_small_size () { # [cfg (target_pointer_width = "64")] assert_eq ! (32 , core :: mem :: size_of ::< State > ()) ; # [cfg (target_pointer_width = "32")] assert_eq ! (16 , core :: mem :: size_of ::< State > ()) ; } }
};
}
