// Generated macro for impl_113 (impl)
macro_rules! Depcrateimpl_113 {
() => {
// Module: crate
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (feature = "cipher")] impl < R : Rounds , V : Variant > StreamCipherSeekCore for ChaChaCore < R , V > { type Counter = V :: Counter ; # [inline (always)] fn get_block_pos (& self) -> Self :: Counter { V :: get_block_pos (& self . state [12 ..]) } # [inline (always)] fn set_block_pos (& mut self , pos : Self :: Counter) { V :: set_block_pos (& mut self . state [12 ..] , pos) ; } }
};
}
