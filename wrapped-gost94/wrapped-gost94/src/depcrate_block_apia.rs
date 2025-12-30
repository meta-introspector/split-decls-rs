// Generated macro for a (function)
macro_rules! Depcrate_block_apia {
() => {
// Module: crate::block_api
// Provides: {"a"}
// Dependencies: {}
fn a (x : Block) -> Block { let mut out = Block :: default () ; out [.. 24] . clone_from_slice (& x [8 ..]) ; for i in 0 .. 8 { out [24 + i] = x [i] ^ x [i + 8] ; } out }
};
}
