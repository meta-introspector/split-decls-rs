// Generated macro for missing_class (function)
macro_rules! Depcratemissing_class {
() => {
// Module: crate
// Provides: {"missing_class"}
// Dependencies: {}
# [test] fn missing_class () { let error = Missing :: new () . unwrap_err () ; assert_eq ! (error . code () , REGDB_E_CLASSNOTREG) ; }
};
}
