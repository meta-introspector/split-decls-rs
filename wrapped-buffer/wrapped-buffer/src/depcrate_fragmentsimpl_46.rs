// Generated macro for impl_46 (impl)
macro_rules! Depcrate_fragmentsimpl_46 {
() => {
// Module: crate::fragments
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'sval , T : ? Sized + Fragment + fmt :: Debug > fmt :: Debug for FragmentBuf < 'sval , T > where T :: Owned : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . value . fmt (f) } }
};
}
