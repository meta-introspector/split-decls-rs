// Generated macro for impl_218 (impl)
macro_rules! Depcrate_escapeimpl_218 {
() => {
// Module: crate::escape
// Provides: {"impl_218"}
// Dependencies: {}
impl std :: fmt :: Display for EscapeError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { Self :: UnrecognizedEntity (rge , res) => { write ! (f , "at {:?}: unrecognized entity `{}`" , rge , res) } Self :: UnterminatedEntity (e) => write ! (f , "Error while escaping character at range {:?}: Cannot find ';' after '&'" , e) , Self :: InvalidCharRef (e) => { write ! (f , "invalid character reference: {}" , e) } } } }
};
}
