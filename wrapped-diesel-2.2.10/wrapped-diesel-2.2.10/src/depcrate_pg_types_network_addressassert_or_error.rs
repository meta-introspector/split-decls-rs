// Generated macro for assert_or_error (macro)
macro_rules! Depcrate_pg_types_network_addressassert_or_error {
() => {
// Module: crate::pg::types::network_address
// Provides: {"assert_or_error"}
// Dependencies: {}
macro_rules ! assert_or_error { ($ cond : expr) => { if !$ cond { return err ! () ; } } ; ($ cond : expr , $ msg : expr) => { if !$ cond { return err ! ($ msg) ; } } ; }
};
}
