// Generated macro for impl_48 (impl)
macro_rules! Depcrate_fragmentsimpl_48 {
() => {
// Module: crate::fragments
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'sval , T : ? Sized + Fragment + PartialEq > PartialEq for FragmentBuf < 'sval , T > where T :: Owned : PartialEq , { fn eq (& self , other : & Self) -> bool { self . value == other . value } }
};
}
