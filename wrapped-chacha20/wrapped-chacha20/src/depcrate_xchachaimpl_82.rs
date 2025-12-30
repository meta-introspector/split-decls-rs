// Generated macro for impl_82 (impl)
macro_rules! Depcrate_xchachaimpl_82 {
() => {
// Module: crate::xchacha
// Provides: {"impl_82"}
// Dependencies: {}
impl < R : Rounds > StreamCipherSeekCore for XChaChaCore < R > { type Counter = u32 ; # [inline (always)] fn get_block_pos (& self) -> u32 { self . 0 . get_block_pos () } # [inline (always)] fn set_block_pos (& mut self , pos : u32) { self . 0 . set_block_pos (pos) ; } }
};
}
