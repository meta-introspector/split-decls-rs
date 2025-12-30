// Generated macro for impl_102 (impl)
macro_rules! Depcrate_utilitiesimpl_102 {
() => {
// Module: crate::utilities
// Provides: {"impl_102"}
// Dependencies: {}
impl < T > fmt :: Display for OptionOsStrDisplay < T > where T : AsRef < OsStr > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (os_str) = self . 0 . as_ref () { write ! (f , "Some({})" , Path :: new (os_str) . display ()) } else { f . write_str ("None") } } }
};
}
