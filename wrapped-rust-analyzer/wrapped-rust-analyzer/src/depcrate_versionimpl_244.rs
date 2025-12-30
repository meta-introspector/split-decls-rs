// Generated macro for impl_244 (impl)
macro_rules! Depcrate_versionimpl_244 {
() => {
// Module: crate::version
// Provides: {"impl_244"}
// Dependencies: {}
impl fmt :: Display for VersionInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . version) ? ; if let Some (ci) = & self . commit_info { write ! (f , " ({} {})" , ci . short_commit_hash , ci . commit_date) ? ; } ; Ok (()) } }
};
}
