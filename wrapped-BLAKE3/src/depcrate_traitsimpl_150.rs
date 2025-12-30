// Generated macro for impl_150 (impl)
macro_rules! Depcrate_traitsimpl_150 {
() => {
// Module: crate::traits
// Provides: {"impl_150"}
// Dependencies: {}
impl digest :: FixedOutput for Hasher { # [inline] fn finalize_into (self , out : & mut GenericArray < u8 , Self :: OutputSize >) { out . copy_from_slice (self . finalize () . as_bytes ()) ; } }
};
}
