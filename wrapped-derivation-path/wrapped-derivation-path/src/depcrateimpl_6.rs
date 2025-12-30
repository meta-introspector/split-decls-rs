// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl fmt :: Display for DerivationPathError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { DerivationPathError :: InvalidDerivationPath (p) => { write ! (f , "invalid derivation path: {p}" ,) } DerivationPathError :: Infallible => f . write_str ("infallible") , } } }
};
}
