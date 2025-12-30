// Generated macro for take_ul (function)
macro_rules! Depcrate_literaltake_ul {
() => {
// Module: crate::literal
// Provides: {"take_ul"}
// Dependencies: {}
fn take_ul (input : & [u8]) -> IResult < & [u8] , & [u8] > { let r = input . split_at_position (| c | c != b'u' && c != b'U' && c != b'l' && c != b'L') ; match r { Err (Err :: Incomplete (_)) => Ok ((& input [input . len () ..] , input)) , res => res , } }
};
}
