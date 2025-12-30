// Generated macro for Iter (struct)
macro_rules! Depcrate_parser_repeatIter {
() => {
// Module: crate::parser::repeat
// Provides: {"Iter"}
// Dependencies: {}
pub struct Iter < 'a , Input , P , S , M > where Input : Stream , P : Parser < Input > , { parser : P , input : & 'a mut Input , committed : bool , state : State < < Input as StreamOnce > :: Error > , partial_state : S , mode : M , }
};
}
