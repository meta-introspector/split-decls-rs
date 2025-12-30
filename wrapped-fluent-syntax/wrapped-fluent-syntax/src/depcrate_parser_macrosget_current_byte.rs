// Generated macro for get_current_byte (macro)
macro_rules! Depcrate_parser_macrosget_current_byte {
() => {
// Module: crate::parser::macros
// Provides: {"get_current_byte"}
// Dependencies: {}
macro_rules ! get_current_byte { ($ s : expr) => { $ s . source . as_ref () . as_bytes () . get ($ s . ptr) } ; }
};
}
