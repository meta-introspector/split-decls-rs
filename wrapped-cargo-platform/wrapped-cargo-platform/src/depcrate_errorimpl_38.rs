// Generated macro for impl_38 (impl)
macro_rules! Depcrate_errorimpl_38 {
() => {
// Module: crate::error
// Provides: {"impl_38"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to parse `{}` as a cfg expression: {}" , self . orig , self . kind) } }
};
}
