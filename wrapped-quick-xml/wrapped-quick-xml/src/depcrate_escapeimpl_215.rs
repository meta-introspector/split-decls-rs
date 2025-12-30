// Generated macro for impl_215 (impl)
macro_rules! Depcrate_escapeimpl_215 {
() => {
// Module: crate::escape
// Provides: {"impl_215"}
// Dependencies: {}
impl std :: fmt :: Display for ParseCharRefError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { Self :: UnexpectedSign => f . write_str ("unexpected number sign") , Self :: InvalidNumber (e) => e . fmt (f) , Self :: InvalidCodepoint (n) => write ! (f , "`{}` is not a valid codepoint" , n) , Self :: IllegalCharacter (n) => write ! (f , "0x{:x} character is not permitted in XML" , n) , } } }
};
}
