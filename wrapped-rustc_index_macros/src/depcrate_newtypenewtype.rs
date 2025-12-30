// Generated macro for newtype (function)
macro_rules! Depcrate_newtypenewtype {
() => {
// Module: crate::newtype
// Provides: {"newtype"}
// Dependencies: {}
pub (crate) fn newtype (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as Newtype) ; input . 0 . into () }
};
}
