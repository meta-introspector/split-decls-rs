// Generated macro for digit (function)
macro_rules! Depcrate_format_parsedigit {
() => {
// Module: crate::format::parse
// Provides: {"digit"}
// Dependencies: {}
# [inline] fn digit (bytes : & [u8 ; 19] , index : usize) -> ParseResult < u8 > { match bytes [index] . is_ascii_digit () { true => Ok (bytes [index] - b'0') , false => Err (INVALID) , } }
};
}
