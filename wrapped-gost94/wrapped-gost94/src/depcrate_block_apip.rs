// Generated macro for p (function)
macro_rules! Depcrate_block_apip {
() => {
// Module: crate::block_api
// Provides: {"p"}
// Dependencies: {}
fn p (y : Block) -> Block { let mut out = Block :: default () ; for i in 0 .. 4 { for k in 0 .. 8 { out [i + 4 * k] = y [8 * i + k] ; } } out }
};
}
