// Generated macro for impl_54 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_54 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_54"}
// Dependencies: {}
impl < NonceSize : Unsigned , Rounds , IsX > StreamCipherSeek for ChaChaAny < NonceSize , Rounds , IsX > { # [inline] fn try_current_pos < T : SeekNum > (& self) -> Result < T , OverflowError > { unimplemented ! () } # [inline (always)] fn try_seek < T : SeekNum > (& mut self , pos : T) -> Result < () , LoopError > { pos . try_into () . map_err (| _ | LoopError) . map (| ct | Self :: seek (self , ct)) } }
};
}
