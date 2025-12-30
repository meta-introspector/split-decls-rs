// Generated macro for impl_117 (impl)
macro_rules! Depcrate_rc_blockimpl_117 {
() => {
// Module: crate::rc_block
// Provides: {"impl_117"}
// Dependencies: {}
impl < F : ? Sized > fmt :: Debug for RcBlock < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("RcBlock") ; let header = unsafe { self . ptr . cast :: < BlockHeader > () . as_ref () } ; debug_block_header (header , & mut f) ; f . finish_non_exhaustive () } }
};
}
