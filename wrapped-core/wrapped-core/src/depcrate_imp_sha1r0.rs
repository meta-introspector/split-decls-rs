// Generated macro for r0 (function)
macro_rules! Depcrate_imp_sha1r0 {
() => {
// Module: crate::imp::sha1
// Provides: {"r0"}
// Dependencies: {}
const fn r0 (block : [u32 ; 16] , v : u32 , mut w : u32 , x : u32 , y : u32 , mut z : u32 , i : usize ,) -> ([u32 ; 16] , u32 , u32) { let n = ((w & (x ^ y)) ^ y) . wrapping_add (block [i]) . wrapping_add (0x5a82_7999) . wrapping_add (rol (v , 5)) ; z = z . wrapping_add (n) ; w = rol (w , 30) ; (block , w , z) }
};
}
