// Generated macro for sha256msg1 (function)
macro_rules! Depcrate_shims_x86_shasha256msg1 {
() => {
// Module: crate::shims::x86::sha
// Provides: {"sha256msg1"}
// Dependencies: {}
fn sha256msg1 (v0 : [u32 ; 4] , v1 : [u32 ; 4]) -> [u32 ; 4] { # [inline] fn sigma0x4 (x : [u32 ; 4]) -> [u32 ; 4] { let t1 = or (shr (x , 7) , shl (x , 25)) ; let t2 = or (shr (x , 18) , shl (x , 14)) ; let t3 = shr (x , 3) ; xor (xor (t1 , t2) , t3) } add (v0 , sigma0x4 (sha256load (v0 , v1))) }
};
}
