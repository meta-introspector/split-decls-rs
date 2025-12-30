// Generated macro for tests (module)
macro_rules! Depcrate_booleantests {
() => {
// Module: crate::boolean
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn to_and_from_bool () { let b_false = CFBoolean :: from (false) ; let b_true = CFBoolean :: from (true) ; assert_ne ! (b_false , b_true) ; assert_eq ! (b_false , CFBoolean :: false_value ()) ; assert_eq ! (b_true , CFBoolean :: true_value ()) ; assert ! (! bool :: from (b_false)) ; assert ! (bool :: from (b_true)) ; } }
};
}
