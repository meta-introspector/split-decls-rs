// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn error_size () { let mut expected_size = core :: mem :: size_of :: < usize > () ; if ! cfg ! (feature = "alloc") { expected_size *= 3 ; } assert_eq ! (expected_size , core :: mem :: size_of ::< Error > ()) ; } }
};
}
