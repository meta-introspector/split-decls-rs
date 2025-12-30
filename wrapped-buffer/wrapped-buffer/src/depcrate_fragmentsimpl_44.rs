// Generated macro for impl_44 (impl)
macro_rules! Depcrate_fragmentsimpl_44 {
() => {
// Module: crate::fragments
// Provides: {"impl_44"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'sval , T : ? Sized + Fragment > Clone for FragmentBuf < 'sval , T > where T :: Owned : Clone , { fn clone (& self) -> Self { FragmentBuf { value : self . value . clone () , } } }
};
}
