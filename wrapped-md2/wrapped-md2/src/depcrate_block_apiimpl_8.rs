// Generated macro for impl_8 (impl)
macro_rules! Depcrate_block_apiimpl_8 {
() => {
// Module: crate::block_api
// Provides: {"impl_8"}
// Dependencies: {}
impl Md2Core { fn compress (& mut self , block : & [u8 ; 16]) { self . x [16 .. 32] . copy_from_slice (block) ; for j in 0 .. 16 { self . x [32 + j] = self . x [16 + j] ^ self . x [j] ; } let mut t = 0u8 ; for j in 0 .. 18u8 { for k in 0 .. STATE_LEN { self . x [k] ^= S [t as usize] ; t = self . x [k] ; } t = t . wrapping_add (j) ; } let mut l = self . checksum [15] ; for j in 0 .. 16 { self . checksum [j] ^= S [(block [j] ^ l) as usize] ; l = self . checksum [j] ; } } }
};
}
