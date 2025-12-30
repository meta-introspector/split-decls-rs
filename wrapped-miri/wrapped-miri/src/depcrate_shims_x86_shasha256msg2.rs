// Generated macro for sha256msg2 (function)
macro_rules! Depcrate_shims_x86_shasha256msg2 {
() => {
// Module: crate::shims::x86::sha
// Provides: {"sha256msg2"}
// Dependencies: {}
fn sha256msg2 (v4 : [u32 ; 4] , v3 : [u32 ; 4]) -> [u32 ; 4] { macro_rules ! sigma1 { ($ a : expr) => { $ a . rotate_right (17) ^ $ a . rotate_right (19) ^ ($ a >> 10) } ; } let [x3 , x2 , x1 , x0] = v4 ; let [w15 , w14 , _ , _] = v3 ; let w16 = x0 . wrapping_add (sigma1 ! (w14)) ; let w17 = x1 . wrapping_add (sigma1 ! (w15)) ; let w18 = x2 . wrapping_add (sigma1 ! (w16)) ; let w19 = x3 . wrapping_add (sigma1 ! (w17)) ; [w19 , w18 , w17 , w16] }
};
}
