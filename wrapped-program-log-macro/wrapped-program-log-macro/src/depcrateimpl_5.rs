// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Parse for LogArgs { fn parse (input : ParseStream) -> syn :: Result < Self > { let buffer_len = if input . peek (LitInt) { let literal = input . parse () ? ; input . parse :: < Token ! [,] > () ? ; literal } else { parse_str :: < LitInt > (DEFAULT_BUFFER_SIZE) ? } ; let format_string = input . parse () ? ; let args = if input . is_empty () { Punctuated :: new () } else { input . parse :: < Token ! [,] > () ? ; Punctuated :: parse_terminated (input) ? } ; Ok (LogArgs { buffer_len , format_string , args , }) } }
};
}
