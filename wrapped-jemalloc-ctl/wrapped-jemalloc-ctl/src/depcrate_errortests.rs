// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn size_of_result_error () { use crate :: mem :: size_of ; assert_eq ! (size_of ::< Result < () >> () , size_of ::< Error > ()) ; assert_eq ! (size_of ::< Error > () , size_of ::< libc :: c_int > ()) ; } }
};
}
