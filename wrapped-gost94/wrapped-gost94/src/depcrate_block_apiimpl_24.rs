// Generated macro for impl_24 (impl)
macro_rules! Depcrate_block_apiimpl_24 {
() => {
// Module: crate::block_api
// Provides: {"impl_24"}
// Dependencies: {}
impl < P : Gost94Params > FixedOutputCore for Gost94Core < P > { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { if buffer . get_pos () != 0 { self . update_n (buffer . get_pos ()) ; self . compress (buffer . pad_with_zeros () . as_ref ()) ; } let mut buf = Block :: default () ; for (o , v) in buf . chunks_exact_mut (8) . zip (self . n . iter ()) { o . copy_from_slice (& v . to_le_bytes ()) ; } self . f (& buf) ; for (o , v) in buf . chunks_exact_mut (8) . zip (self . sigma . iter ()) { o . copy_from_slice (& v . to_le_bytes ()) ; } self . f (& buf) ; out . copy_from_slice (& self . h) ; } }
};
}
