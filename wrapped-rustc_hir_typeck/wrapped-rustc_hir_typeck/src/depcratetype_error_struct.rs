// Generated macro for type_error_struct (macro)
macro_rules! Depcratetype_error_struct {
() => {
// Module: crate
// Provides: {"type_error_struct"}
// Dependencies: {}
# [macro_export] macro_rules ! type_error_struct { ($ dcx : expr , $ span : expr , $ typ : expr , $ code : expr , $ ($ message : tt) *) => ({ let mut err = rustc_errors :: struct_span_code_err ! ($ dcx , $ span , $ code , $ ($ message) *) ; if $ typ . references_error () { err . downgrade_to_delayed_bug () ; } err }) }
};
}
