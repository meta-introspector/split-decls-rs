// Generated macro for impl_217 (impl)
macro_rules! Depcrate_envimpl_217 {
() => {
// Module: crate::env
// Provides: {"impl_217"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Deserialize (_) => write ! (f , "failed to deserialize tool config") , Error :: Env (var) => write ! (f , "invalid value for env var '{var}'") , Error :: Config (var) => write ! (f , "invalid value for config '{var}'") , } } }
};
}
