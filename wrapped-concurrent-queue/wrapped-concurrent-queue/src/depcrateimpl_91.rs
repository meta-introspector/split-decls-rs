// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > fmt :: Display for PushError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { PushError :: Full (_) => write ! (f , "Full") , PushError :: Closed (_) => write ! (f , "Closed") , } } }
};
}
