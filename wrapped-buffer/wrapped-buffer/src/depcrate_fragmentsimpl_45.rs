// Generated macro for impl_45 (impl)
macro_rules! Depcrate_fragmentsimpl_45 {
() => {
// Module: crate::fragments
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] impl < 'sval , T : ? Sized + Fragment + fmt :: Debug > fmt :: Debug for FragmentBuf < 'sval , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . value . fmt (f) } }
};
}
