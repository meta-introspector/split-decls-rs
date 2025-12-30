// Generated macro for err_expected (macro)
macro_rules! Depcrate_deerr_expected {
() => {
// Module: crate::de
// Provides: {"err_expected"}
// Dependencies: {}
macro_rules ! err_expected { ($ expected : literal , $ got : expr) => { Err (Error :: DeserializationError (format ! ("Expected {}, found '{:?}'" , $ expected , $ got))) } ; }
};
}
