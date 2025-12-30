// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl StreamCipherBackend for Backend < '_ > { # [inline (always)] fn gen_ks_block (& mut self , block : & mut Block < Self >) { block . copy_from_slice (& self . 0 . next_block ()) ; } }
};
}
