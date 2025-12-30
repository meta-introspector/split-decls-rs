// Generated macro for impl_263 (impl)
macro_rules! Depcrate_options_errorimpl_263 {
() => {
// Module: crate::options::error
// Provides: {"impl_263"}
// Dependencies: {}
impl fmt :: Display for NumberSource { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Arg (arg) => write ! (f , "option {arg}") , Self :: Env (env) => write ! (f , "environment variable {env}") , } } }
};
}
