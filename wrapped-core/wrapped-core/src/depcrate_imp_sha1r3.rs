// Generated macro for r3 (function)
macro_rules! Depcrate_imp_sha1r3 {
() => {
// Module: crate::imp::sha1
// Provides: {"r3"}
// Dependencies: {}
const fn r3 (mut block : [u32 ; 16] , v : u32 , mut w : u32 , x : u32 , y : u32 , mut z : u32 , i : usize ,) -> ([u32 ; 16] , u32 , u32) { block [i] = blk (& block , i) ; let n = (((w | x) & y) | (w & x)) . wrapping_add (block [i]) . wrapping_add (0x8f1b_bcdc) . wrapping_add (rol (v , 5)) ; z = z . wrapping_add (n) ; w = rol (w , 30) ; (block , w , z) }
};
}
