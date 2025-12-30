// Generated macro for impl_559 (impl)
macro_rules! Depcrate_enc_writeimpl_559 {
() => {
// Module: crate::enc::write
// Provides: {"impl_559"}
// Dependencies: {}
impl < T : Writer > Writer for & mut T { # [inline] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { (* * self) . write (bytes) } }
};
}
