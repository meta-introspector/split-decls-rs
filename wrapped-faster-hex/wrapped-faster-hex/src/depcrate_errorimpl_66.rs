// Generated macro for impl_66 (impl)
macro_rules! Depcrate_errorimpl_66 {
() => {
// Module: crate::error
// Provides: {"impl_66"}
// Dependencies: {}
impl :: core :: fmt :: Debug for Error { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { match * self { Error :: InvalidLength (len) => write ! (f , "Invalid input length {len}") , Error :: InvalidChar => write ! (f , "Invalid character") , Error :: Overflow => write ! (f , "Overflow") , } } }
};
}
