// Generated macro for hexadecimal (function)
macro_rules! Depcrate_literalhexadecimal {
() => {
// Module: crate::literal
// Provides: {"hexadecimal"}
// Dependencies: {}
fn hexadecimal (i : & [u8]) -> nom :: IResult < & [u8] , u8 > { byte ! (b'0' ..= b'9' | b'a' ..= b'f' | b'A' ..= b'F') (i) }
};
}
