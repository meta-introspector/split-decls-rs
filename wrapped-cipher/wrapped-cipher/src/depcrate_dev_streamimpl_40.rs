// Generated macro for impl_40 (impl)
macro_rules! Depcrate_dev_streamimpl_40 {
() => {
// Module: crate::dev::stream
// Provides: {"impl_40"}
// Dependencies: {}
impl < C : StreamCipher > StreamCipher for & mut C { # [inline] fn check_remaining (& self , data_len : usize) -> Result < () , StreamCipherError > { C :: check_remaining (self , data_len) } # [inline] fn unchecked_apply_keystream_inout (& mut self , buf : InOutBuf < '_ , '_ , u8 >) { C :: unchecked_apply_keystream_inout (self , buf) } # [inline] fn unchecked_write_keystream (& mut self , buf : & mut [u8]) { C :: unchecked_write_keystream (self , buf) } }
};
}
