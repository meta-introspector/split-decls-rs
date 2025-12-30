// Generated macro for impl_90 (impl)
macro_rules! Depcrate_lockfileimpl_90 {
() => {
// Module: crate::lockfile
// Provides: {"impl_90"}
// Dependencies: {}
impl fmt :: Display for TomlLockfilePackageId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name) ? ; if let Some (s) = & self . version { write ! (f , " {}" , s) ? ; } if let Some (s) = & self . source { write ! (f , " ({})" , s . as_url ()) ? ; } Ok (()) } }
};
}
