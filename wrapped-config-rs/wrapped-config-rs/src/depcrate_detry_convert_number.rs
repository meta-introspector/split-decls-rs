// Generated macro for try_convert_number (macro)
macro_rules! Depcrate_detry_convert_number {
() => {
// Module: crate::de
// Provides: {"try_convert_number"}
// Dependencies: {}
macro_rules ! try_convert_number { (signed , $ self : expr , $ size : literal) => { { let num = $ self . into_int () ?; num . try_into () . map_err (| _ | { ConfigError :: invalid_type (None , Unexpected :: I64 (num) , concat ! ("an signed " , $ size , " bit integer") ,) }) ? } } ; (unsigned , $ self : expr , $ size : literal) => { { let num = $ self . into_uint () ?; num . try_into () . map_err (| _ | { ConfigError :: invalid_type (None , Unexpected :: U64 (num) , concat ! ("an unsigned " , $ size , " bit integer") ,) }) ? } } ; }
};
}
