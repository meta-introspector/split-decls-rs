// Generated macro for parse_err (function)
macro_rules! Depcrate_bool_testsparse_err {
() => {
// Module: crate::bool::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { assert ! (Literal :: parse ("fa") . is_err ()) ; assert ! (Literal :: parse ("fal") . is_err ()) ; assert ! (Literal :: parse ("fals") . is_err ()) ; assert ! (Literal :: parse (" false") . is_err ()) ; assert ! (Literal :: parse ("false ") . is_err ()) ; assert ! (Literal :: parse ("False") . is_err ()) ; assert ! (Literal :: parse ("tr") . is_err ()) ; assert ! (Literal :: parse ("tru") . is_err ()) ; assert ! (Literal :: parse (" true") . is_err ()) ; assert ! (Literal :: parse ("true ") . is_err ()) ; assert ! (Literal :: parse ("True") . is_err ()) ; }
};
}
