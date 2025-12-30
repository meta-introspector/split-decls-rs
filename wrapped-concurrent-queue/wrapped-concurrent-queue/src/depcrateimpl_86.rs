// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl fmt :: Display for PopError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { PopError :: Empty => write ! (f , "Empty") , PopError :: Closed => write ! (f , "Closed") , } } }
};
}
