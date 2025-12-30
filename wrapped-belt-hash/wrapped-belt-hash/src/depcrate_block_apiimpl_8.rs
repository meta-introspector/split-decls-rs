// Generated macro for impl_8 (impl)
macro_rules! Depcrate_block_apiimpl_8 {
() => {
// Module: crate::block_api
// Provides: {"impl_8"}
// Dependencies: {}
impl BeltHashCore { fn compress_block (& mut self , block : & Block < Self >) { let x1 = read_u32s (& block [.. 16]) ; let x2 = read_u32s (& block [16 ..]) ; let (t , h) = belt_compress (x1 , x2 , self . h) ; self . h = h ; self . s = xor (self . s , t) ; } }
};
}
