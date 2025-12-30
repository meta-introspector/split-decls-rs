// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [allow (deprecated , unused_imports)] # [cfg (test)] mod tests { use crate :: error :: KeyRejected ; use crate :: test ; use std :: error :: Error ; # [test] fn display_unspecified () { let output = format ! ("{}" , super :: Unspecified) ; assert_eq ! ("Unspecified" , output) ; } # [test] fn unexpected_error () { let key_rejected = super :: KeyRejected :: from (()) ; assert_eq ! ("UnexpectedError" , key_rejected . description ()) ; let unspecified = super :: Unspecified :: from (key_rejected) ; assert_eq ! ("Unspecified" , unspecified . description ()) ; # [allow (clippy :: redundant_locals)] let unspecified = unspecified ; assert_eq ! ("Unspecified" , unspecified . description ()) ; } # [test] fn std_error () { let key_rejected = KeyRejected :: wrong_algorithm () ; assert ! (key_rejected . cause () . is_none ()) ; assert_eq ! ("WrongAlgorithm" , key_rejected . description ()) ; let unspecified = super :: Unspecified ; assert ! (unspecified . cause () . is_none ()) ; assert_eq ! ("Unspecified" , unspecified . description ()) ; test :: compile_time_assert_std_error_error :: < KeyRejected > () ; } }
};
}
