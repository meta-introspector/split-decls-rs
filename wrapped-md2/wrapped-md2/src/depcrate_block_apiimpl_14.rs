// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl FixedOutputCore for Md2Core { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let rem = buffer . remaining () as u8 ; let mut block = buffer . pad_with_zeros () ; block [pos ..] . iter_mut () . for_each (| b | * b = rem) ; self . compress (block . as_ref ()) ; let checksum = self . checksum ; self . compress (& checksum) ; out . copy_from_slice (& self . x [.. 16]) ; } }
};
}
