// Generated macro for impl_331 (impl)
macro_rules! Depcrate_parser_lexerimpl_331 {
() => {
// Module: crate::parser::lexer
// Provides: {"impl_331"}
// Dependencies: {}
impl Display for UnicodeCodePoint { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_variable_width { write ! (f , r"\u{{{:X}}}" , self . code) } else { write ! (f , r"\u{:04X}" , self . code) } } }
};
}
