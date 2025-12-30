// Generated macro for impl_398 (impl)
macro_rules! Depcrate_lspimpl_398 {
() => {
// Module: crate::lsp
// Provides: {"impl_398"}
// Dependencies: {}
impl fmt :: Display for LspError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Language Server request failed with {}. ({})" , self . code , self . message) } }
};
}
