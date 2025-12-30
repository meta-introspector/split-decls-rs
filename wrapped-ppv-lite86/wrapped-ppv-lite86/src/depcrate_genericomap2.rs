// Generated macro for omap2 (function)
macro_rules! Depcrate_genericomap2 {
() => {
// Module: crate::generic
// Provides: {"omap2"}
// Dependencies: {}
# [inline (always)] fn omap2 < T , F > (a : T , b : T , f : F) -> T where T : Store < vec128_storage > + Into < vec128_storage > , F : Fn (u128 , u128) -> u128 , { let a : vec128_storage = a . into () ; let b : vec128_storage = b . into () ; let ao = o_of_q (unsafe { a . q }) ; let bo = o_of_q (unsafe { b . q }) ; let o = vec128_storage { q : q_of_o (f (ao , bo)) , } ; unsafe { T :: unpack (o) } }
};
}
