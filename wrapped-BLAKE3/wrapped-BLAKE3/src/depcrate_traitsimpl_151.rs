// Generated macro for impl_151 (impl)
macro_rules! Depcrate_traitsimpl_151 {
() => {
// Module: crate::traits
// Provides: {"impl_151"}
// Dependencies: {}
impl digest :: FixedOutputReset for Hasher { # [inline] fn finalize_into_reset (& mut self , out : & mut GenericArray < u8 , Self :: OutputSize >) { out . copy_from_slice (self . finalize () . as_bytes ()) ; self . reset () ; } }
};
}
