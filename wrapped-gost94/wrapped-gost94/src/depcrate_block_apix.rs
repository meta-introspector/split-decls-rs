// Generated macro for x (function)
macro_rules! Depcrate_block_apix {
() => {
// Module: crate::block_api
// Provides: {"x"}
// Dependencies: {}
fn x (a : & Block , b : & Block) -> Block { let mut out = Block :: default () ; for i in 0 .. 32 { out [i] = a [i] ^ b [i] ; } out }
};
}
