// Generated macro for inner_u64 (function)
macro_rules! Depcrate_utilinner_u64 {
() => {
// Module: crate::util
// Provides: {"inner_u64"}
// Dependencies: {}
# [doc = " Default implementation of `inner_u64` on top of `fill_uninit`"] # [inline] pub fn inner_u64 () -> Result < u64 , Error > { let mut res = MaybeUninit :: < u64 > :: uninit () ; let dst = unsafe { let p : * mut MaybeUninit < u8 > = res . as_mut_ptr () . cast () ; slice :: from_raw_parts_mut (p , core :: mem :: size_of :: < u64 > ()) } ; crate :: fill_uninit (dst) ? ; Ok (unsafe { res . assume_init () }) }
};
}
