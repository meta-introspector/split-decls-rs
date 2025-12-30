// Generated macro for get_byte (macro)
macro_rules! Depcrate_parser_macrosget_byte {
() => {
// Module: crate::parser::macros
// Provides: {"get_byte"}
// Dependencies: {}
macro_rules ! get_byte { ($ s : expr , $ idx : expr) => { $ s . source . as_ref () . as_bytes () . get ($ idx) } ; }
};
}
