// Generated macro for c_char (function)
macro_rules! Depcrate_literalc_char {
() => {
// Module: crate::literal
// Provides: {"c_char"}
// Dependencies: {}
fn c_char (i : & [u8]) -> nom :: IResult < & [u8] , CChar > { delimited (terminated (opt (c_width_prefix) , char ('\'')) , alt ((escaped_char , map (byte ! (0 ..= 91 | 93 ..= 255) , CChar :: from) ,)) , char ('\'') ,) (i) }
};
}
