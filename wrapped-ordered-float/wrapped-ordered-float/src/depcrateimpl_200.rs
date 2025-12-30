// Generated macro for impl_200 (impl)
macro_rules! Depcrateimpl_200 {
() => {
// Module: crate
// Provides: {"impl_200"}
// Dependencies: {}
impl < E : fmt :: Display > fmt :: Display for ParseNotNanError < E > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { ParseNotNanError :: ParseFloatError (e) => write ! (f , "Parse error: {e}") , ParseNotNanError :: IsNaN => write ! (f , "NotNan parser encounter a NaN") , } } }
};
}
