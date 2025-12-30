// Generated macro for impl_28 (impl)
macro_rules! Depcrate_murmur3impl_28 {
() => {
// Module: crate::murmur3
// Provides: {"impl_28"}
// Dependencies: {}
impl State { # [allow (clippy :: trivially_copy_pass_by_ref)] fn process_block (& mut self , block : & MaybeUninit < [u8 ; 4] >) { self . 0 ^= pre_mix (u32 :: from_le_bytes (unsafe { * block . assume_init_ref () })) ; self . 0 = self . 0 . rotate_left (13) ; self . 0 = 5u32 . wrapping_mul (self . 0) . wrapping_add (0xe6546b64) ; } }
};
}
