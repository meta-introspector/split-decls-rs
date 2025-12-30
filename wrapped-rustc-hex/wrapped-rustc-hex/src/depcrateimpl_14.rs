// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl fmt :: Debug for FromHexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { InvalidHexCharacter (ch , idx) => write ! (f , "Invalid character '{}' at position {}" , ch , idx) , InvalidHexLength => write ! (f , "Invalid input length") , } } }
};
}
