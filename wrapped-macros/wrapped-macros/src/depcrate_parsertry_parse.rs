// Generated macro for try_parse (function)
macro_rules! Depcrate_parsertry_parse {
() => {
// Module: crate::parser
// Provides: {"try_parse"}
// Dependencies: {}
# [inline] pub const fn try_parse (input : & '_ str) -> Result < [u8 ; 16] , InvalidUuid < '_ > > { let result = match (input . len () , input . as_bytes ()) { (32 , s) => parse_simple (s) , (36 , s) | (38 , [b'{' , s @ .. , b'}']) | (45 , [b'u' , b'r' , b'n' , b':' , b'u' , b'u' , b'i' , b'd' , b':' , s @ ..]) => { parse_hyphenated (s) } _ => Err (()) , } ; match result { Ok (b) => Ok (b) , Err (()) => Err (InvalidUuid (input)) , } }
};
}
