// Generated macro for c_float (function)
macro_rules! Depcrate_literalc_float {
() => {
// Module: crate::literal
// Provides: {"c_float"}
// Dependencies: {}
fn c_float (i : & [u8]) -> nom :: IResult < & [u8] , f64 > { map_opt (alt ((terminated (recognize (tuple ((many1 (complete (decimal)) , byte ! (b'.') , many0 (complete (decimal)) ,))) , opt (float_width) ,) , terminated (recognize (tuple ((many0 (complete (decimal)) , byte ! (b'.') , many1 (complete (decimal)) ,))) , opt (float_width) ,) , terminated (recognize (tuple ((many0 (complete (decimal)) , opt (byte ! (b'.')) , many1 (complete (decimal)) , float_exp ,))) , opt (float_width) ,) , terminated (recognize (tuple ((many1 (complete (decimal)) , opt (byte ! (b'.')) , many0 (complete (decimal)) , float_exp ,))) , opt (float_width) ,) , terminated (recognize (many1 (complete (decimal))) , float_width) ,)) , | v | str :: from_utf8 (v) . ok () . and_then (| i | f64 :: from_str (i) . ok ()) ,) (i) }
};
}
