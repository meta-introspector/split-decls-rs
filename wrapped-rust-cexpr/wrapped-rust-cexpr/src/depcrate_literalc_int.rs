// Generated macro for c_int (function)
macro_rules! Depcrate_literalc_int {
() => {
// Module: crate::literal
// Provides: {"c_int"}
// Dependencies: {}
fn c_int (i : & [u8]) -> nom :: IResult < & [u8] , i64 > { map (terminated (alt ((map_opt (preceded (tag ("0x") , many1 (complete (hexadecimal))) , | v | { c_int_radix (v , 16) }) , map_opt (preceded (tag ("0X") , many1 (complete (hexadecimal))) , | v | { c_int_radix (v , 16) }) , map_opt (preceded (tag ("0b") , many1 (complete (binary))) , | v | { c_int_radix (v , 2) }) , map_opt (preceded (tag ("0B") , many1 (complete (binary))) , | v | { c_int_radix (v , 2) }) , map_opt (preceded (char ('0') , many1 (complete (octal))) , | v | { c_int_radix (v , 8) }) , map_opt (many1 (complete (decimal)) , | v | c_int_radix (v , 10)) , | input | Err (crate :: nom :: Err :: Error (nom :: error :: Error :: new (input , crate :: nom :: ErrorKind :: Fix))) ,)) , opt (take_ul) ,) , | i | i as i64 ,) (i) }
};
}
