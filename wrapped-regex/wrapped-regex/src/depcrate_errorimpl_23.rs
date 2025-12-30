// Generated macro for impl_23 (impl)
macro_rules! Depcrate_errorimpl_23 {
() => {
// Module: crate::error
// Provides: {"impl_23"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { Error :: Syntax (ref err) => err . fmt (f) , Error :: CompiledTooBig (limit) => write ! (f , "Compiled regex exceeds size limit of {limit} bytes." ,) , } } }
};
}
