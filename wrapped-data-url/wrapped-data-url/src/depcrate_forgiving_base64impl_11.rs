// Generated macro for impl_11 (impl)
macro_rules! Depcrate_forgiving_base64impl_11 {
() => {
// Module: crate::forgiving_base64
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Display for InvalidBase64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 0 { InvalidBase64Details :: UnexpectedSymbol (code_point) => { write ! (f , "symbol with codepoint {} not expected" , code_point) } InvalidBase64Details :: AlphabetSymbolAfterPadding => { write ! (f , "alphabet symbol present after padding") } InvalidBase64Details :: LoneAlphabetSymbol => write ! (f , "lone alphabet symbol present") , InvalidBase64Details :: Padding => write ! (f , "incorrect padding") , } } }
};
}
