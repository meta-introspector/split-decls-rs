// Generated macro for psi (function)
macro_rules! Depcrate_block_apipsi {
() => {
// Module: crate::block_api
// Provides: {"psi"}
// Dependencies: {}
fn psi (block : & mut Block) { let mut out = Block :: default () ; out [.. 30] . copy_from_slice (& block [2 ..]) ; out [30 ..] . copy_from_slice (& block [.. 2]) ; out [30] ^= block [2] ; out [31] ^= block [3] ; out [30] ^= block [4] ; out [31] ^= block [5] ; out [30] ^= block [6] ; out [31] ^= block [7] ; out [30] ^= block [24] ; out [31] ^= block [25] ; out [30] ^= block [30] ; out [31] ^= block [31] ; block . copy_from_slice (& out) ; }
};
}
