// Generated macro for recursion_limit (function)
macro_rules! Depcrate_ser_testsrecursion_limit {
() => {
// Module: crate::ser::tests
// Provides: {"recursion_limit"}
// Dependencies: {}
# [test] fn recursion_limit () { assert_eq ! (crate :: Options :: default () . with_recursion_limit (0) . to_string (& [42]) , Err (crate :: Error :: ExceededRecursionLimit) ,) ; assert_eq ! (crate :: Options :: default () . with_recursion_limit (1) . to_string (& [42]) . as_deref () , Ok ("(42)") ,) ; assert_eq ! (crate :: Options :: default () . without_recursion_limit () . to_string (& [42]) . as_deref () , Ok ("(42)") ,) ; assert_eq ! (crate :: Options :: default () . with_recursion_limit (1) . to_string (& [[42]]) , Err (crate :: Error :: ExceededRecursionLimit) ,) ; assert_eq ! (crate :: Options :: default () . with_recursion_limit (2) . to_string (& [[42]]) . as_deref () , Ok ("((42))") ,) ; assert_eq ! (crate :: Options :: default () . without_recursion_limit () . to_string (& [[42]]) . as_deref () , Ok ("((42))") ,) ; }
};
}
