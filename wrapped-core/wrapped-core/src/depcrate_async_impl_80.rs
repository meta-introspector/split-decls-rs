// Generated macro for impl_80 (impl)
macro_rules! Depcrate_async_impl_80 {
() => {
// Module: crate::async_
// Provides: {"impl_80"}
// Dependencies: {}
impl fmt :: Display for AsyncFilter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AsyncFilter :: All => write ! (f , "all") , AsyncFilter :: Function (s) => write ! (f , "{s}") , AsyncFilter :: Import (s) => write ! (f , "import:{s}") , AsyncFilter :: Export (s) => write ! (f , "export:{s}") , } } }
};
}
