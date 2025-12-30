// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl StreamCipherBackend for Backend < '_ > { # [inline (always)] fn gen_ks_block (& mut self , block : & mut Block < Self >) { block . copy_from_slice (& self . 0 . gen_word () . to_le_bytes ()) ; } }
};
}
