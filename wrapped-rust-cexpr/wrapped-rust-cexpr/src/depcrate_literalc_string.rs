// Generated macro for c_string (function)
macro_rules! Depcrate_literalc_string {
() => {
// Module: crate::literal
// Provides: {"c_string"}
// Dependencies: {}
fn c_string (i : & [u8]) -> nom :: IResult < & [u8] , Vec < u8 > > { delimited (alt ((preceded (c_width_prefix , char ('"')) , char ('"'))) , fold_many0 (alt ((map (escaped_char , | c : CChar | c . into ()) , map (is_not ([b'\\' , b'"']) , | c : & [u8] | c . into ()) ,)) , Vec :: new , | mut v : Vec < u8 > , res : Vec < u8 > | { v . extend_from_slice (& res) ; v } ,) , char ('"') ,) (i) }
};
}
