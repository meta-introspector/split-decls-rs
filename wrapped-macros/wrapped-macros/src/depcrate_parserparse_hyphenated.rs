// Generated macro for parse_hyphenated (function)
macro_rules! Depcrate_parserparse_hyphenated {
() => {
// Module: crate::parser
// Provides: {"parse_hyphenated"}
// Dependencies: {}
# [inline] const fn parse_hyphenated (s : & [u8]) -> Result < [u8 ; 16] , () > { if s . len () != 36 { return Err (()) ; } match [s [8] , s [13] , s [18] , s [23]] { [b'-' , b'-' , b'-' , b'-'] => { } _ => return Err (()) , } let positions : [u8 ; 8] = [0 , 4 , 9 , 14 , 19 , 24 , 28 , 32] ; let mut buf : [u8 ; 16] = [0 ; 16] ; let mut j = 0 ; while j < 8 { let i = positions [j] ; let h1 = HEX_TABLE [s [i as usize] as usize] ; let h2 = HEX_TABLE [s [(i + 1) as usize] as usize] ; let h3 = HEX_TABLE [s [(i + 2) as usize] as usize] ; let h4 = HEX_TABLE [s [(i + 3) as usize] as usize] ; if h1 | h2 | h3 | h4 == 0xff { return Err (()) ; } buf [j * 2] = SHL4_TABLE [h1 as usize] | h2 ; buf [j * 2 + 1] = SHL4_TABLE [h3 as usize] | h4 ; j += 1 ; } Ok (buf) }
};
}
