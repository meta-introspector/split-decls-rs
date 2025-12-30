// Generated macro for impl_738 (impl)
macro_rules! Depcrate_csrimpl_738 {
() => {
// Module: crate::csr
// Provides: {"impl_738"}
// Dependencies: {}
impl fmt :: Display for CsrError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CsrError :: IndicesOutBounds (a , b) => { write ! (f , "Both node indices {a} and {b} is out of Csr bounds") } } } }
};
}
