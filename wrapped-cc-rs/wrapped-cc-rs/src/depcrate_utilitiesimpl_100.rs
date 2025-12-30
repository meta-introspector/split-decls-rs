// Generated macro for impl_100 (impl)
macro_rules! Depcrate_utilitiesimpl_100 {
() => {
// Module: crate::utilities
// Provides: {"impl_100"}
// Dependencies: {}
impl < T > fmt :: Display for JoinOsStrs < '_ , T > where T : AsRef < OsStr > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let len = self . slice . len () ; for (index , os_str) in self . slice . iter () . enumerate () { write ! (f , "{}" , Path :: new (os_str) . display ()) ? ; if index + 1 < len { f . write_char (self . delimiter) ? ; } } Ok (()) } }
};
}
