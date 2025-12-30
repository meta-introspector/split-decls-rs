// Generated macro for decimal (function)
macro_rules! Depcrate_literaldecimal {
() => {
// Module: crate::literal
// Provides: {"decimal"}
// Dependencies: {}
fn decimal (i : & [u8]) -> nom :: IResult < & [u8] , u8 > { byte ! (b'0' ..= b'9') (i) }
};
}
