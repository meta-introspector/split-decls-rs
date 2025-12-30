// Generated macro for sha256_digest_round_x2 (function)
macro_rules! Depcrate_shims_x86_shasha256_digest_round_x2 {
() => {
// Module: crate::shims::x86::sha
// Provides: {"sha256_digest_round_x2"}
// Dependencies: {}
fn sha256_digest_round_x2 (cdgh : [u32 ; 4] , abef : [u32 ; 4] , wk : [u32 ; 4]) -> [u32 ; 4] { macro_rules ! big_sigma0 { ($ a : expr) => { ($ a . rotate_right (2) ^ $ a . rotate_right (13) ^ $ a . rotate_right (22)) } ; } macro_rules ! big_sigma1 { ($ a : expr) => { ($ a . rotate_right (6) ^ $ a . rotate_right (11) ^ $ a . rotate_right (25)) } ; } macro_rules ! bool3ary_202 { ($ a : expr , $ b : expr , $ c : expr) => { $ c ^ ($ a & ($ b ^ $ c)) } ; } macro_rules ! bool3ary_232 { ($ a : expr , $ b : expr , $ c : expr) => { ($ a & $ b) ^ ($ a & $ c) ^ ($ b & $ c) } ; } let [_ , _ , wk1 , wk0] = wk ; let [a0 , b0 , e0 , f0] = abef ; let [c0 , d0 , g0 , h0] = cdgh ; let x0 = big_sigma1 ! (e0) . wrapping_add (bool3ary_202 ! (e0 , f0 , g0)) . wrapping_add (wk0) . wrapping_add (h0) ; let y0 = big_sigma0 ! (a0) . wrapping_add (bool3ary_232 ! (a0 , b0 , c0)) ; let (a1 , b1 , c1 , d1 , e1 , f1 , g1 , h1) = (x0 . wrapping_add (y0) , a0 , b0 , c0 , x0 . wrapping_add (d0) , e0 , f0 , g0) ; let x1 = big_sigma1 ! (e1) . wrapping_add (bool3ary_202 ! (e1 , f1 , g1)) . wrapping_add (wk1) . wrapping_add (h1) ; let y1 = big_sigma0 ! (a1) . wrapping_add (bool3ary_232 ! (a1 , b1 , c1)) ; let (a2 , b2 , _ , _ , e2 , f2 , _ , _) = (x1 . wrapping_add (y1) , a1 , b1 , c1 , x1 . wrapping_add (d1) , e1 , f1 , g1) ; [a2 , b2 , e2 , f2] }
};
}
