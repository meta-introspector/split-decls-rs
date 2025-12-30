// Generated macro for impl_45 (impl)
macro_rules! Depcrate_blockimpl_45 {
() => {
// Module: crate::block
// Provides: {"impl_45"}
// Dependencies: {}
impl < F : ? Sized > fmt :: Debug for Block < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("Block") ; debug_block_header (self . header () , & mut f) ; f . finish_non_exhaustive () } }
};
}
