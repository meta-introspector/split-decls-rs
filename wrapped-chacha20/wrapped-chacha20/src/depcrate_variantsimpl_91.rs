// Generated macro for impl_91 (impl)
macro_rules! Depcrate_variantsimpl_91 {
() => {
// Module: crate::variants
// Provides: {"impl_91"}
// Dependencies: {}
impl Variant for Ietf { type Counter = u32 ; # [inline (always)] fn get_block_pos (row : & [u32]) -> u32 { row [0] } # [inline (always)] fn set_block_pos (row : & mut [u32] , pos : u32) { row [0] = pos ; } # [inline (always)] fn remaining_blocks (block_pos : u32) -> Option < usize > { let remaining = u32 :: MAX - block_pos ; remaining . try_into () . ok () } }
};
}
