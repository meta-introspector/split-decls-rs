// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl fmt :: Display for FromHexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { FromHexError :: InvalidHexCharacter { c , index } => { write ! (f , "Invalid character {c:?} at position {index}") } FromHexError :: OddLength => write ! (f , "Odd number of digits") , FromHexError :: InvalidStringLength => write ! (f , "Invalid string length") , } } }
};
}
