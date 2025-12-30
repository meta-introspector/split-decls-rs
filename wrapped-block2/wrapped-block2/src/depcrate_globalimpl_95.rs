// Generated macro for impl_95 (impl)
macro_rules! Depcrate_globalimpl_95 {
() => {
// Module: crate::global
// Provides: {"impl_95"}
// Dependencies: {}
impl < F : ? Sized > fmt :: Debug for GlobalBlock < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("GlobalBlock") ; debug_block_header (& self . header , & mut f) ; f . finish_non_exhaustive () } }
};
}
