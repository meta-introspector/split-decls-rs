// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { ErrorClass , ErrorCode } ; # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let err = repo . find_submodule ("does_not_exist") . err () . unwrap () ; assert_eq ! (err . code () , ErrorCode :: NotFound) ; assert_eq ! (err . class () , ErrorClass :: Submodule) ; } }
};
}
