// Generated macro for sha1 (function)
macro_rules! Depcrate_imp_sha1sha1 {
() => {
// Module: crate::imp::sha1
// Provides: {"sha1"}
// Dependencies: {}
pub const fn sha1 (data : & ConstBuffer) -> Digest { let state : [u32 ; 5] = [0x67452301 , 0xefcdab89 , 0x98badcfe , 0x10325476 , 0xc3d2e1f0] ; let len : u64 = 0 ; let blocks = Blocks { len : 0 , data : [0 ; 64] , } ; let (blocks , len , state) = process_blocks (blocks , data , len , state) ; digest (state , len , blocks) }
};
}
