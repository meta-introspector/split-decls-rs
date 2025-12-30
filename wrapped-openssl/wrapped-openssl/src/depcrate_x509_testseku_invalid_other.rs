// Generated macro for eku_invalid_other (function)
macro_rules! Depcrate_x509_testseku_invalid_other {
() => {
// Module: crate::x509::tests
// Provides: {"eku_invalid_other"}
// Dependencies: {}
# [test] fn eku_invalid_other () { assert ! (ExtendedKeyUsage :: new () . other ("1.1.1.1.1,2.2.2.2.2") . build () . is_err ()) ; }
};
}
