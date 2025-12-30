// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Error ; use crate :: Reason ; # [test] fn error_from_reason () { let err = Error :: from (Reason :: HTTP_1_1_REQUIRED) ; assert_eq ! (err . reason () , Some (Reason :: HTTP_1_1_REQUIRED)) ; } }
};
}
