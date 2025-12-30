// Generated macro for impl_139 (impl)
macro_rules! Depcrate_tests_parserimpl_139 {
() => {
// Module: crate::tests::parser
// Provides: {"impl_139"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { ParseErrorKind :: InvalidNamedFlag { got } => { let _got = got ; write ! (f , "unrecognized named flag") ? ; # [cfg (feature = "std")] { write ! (f , " `{}`" , _got) ? ; } } ParseErrorKind :: InvalidHexFlag { got } => { let _got = got ; write ! (f , "invalid hex flag") ? ; # [cfg (feature = "std")] { write ! (f , " `{}`" , _got) ? ; } } ParseErrorKind :: EmptyFlag => { write ! (f , "encountered empty flag") ? ; } } Ok (()) } }
};
}
