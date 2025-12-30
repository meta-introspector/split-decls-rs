// Generated macro for octal (function)
macro_rules! Depcrate_literaloctal {
() => {
// Module: crate::literal
// Provides: {"octal"}
// Dependencies: {}
fn octal (i : & [u8]) -> nom :: IResult < & [u8] , u8 > { byte ! (b'0' ..= b'7') (i) }
};
}
