// Generated macro for impl_3159 (impl)
macro_rules! Depcrate_incremental_cacheimpl_3159 {
() => {
// Module: crate::incremental_cache
// Provides: {"impl_3159"}
// Dependencies: {}
impl fmt :: Display for RecompileError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { RecompileError :: VersionMismatch => write ! (f , "cranelift version mismatch" ,) , RecompileError :: Deserialize (err) => { write ! (f , "postcard failed during deserialization: {err}") } } } }
};
}
