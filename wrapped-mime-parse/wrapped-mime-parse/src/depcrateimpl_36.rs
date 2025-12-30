// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let description = match self { ParseError :: MissingSlash => "a slash (/) was missing between the type and subtype" , ParseError :: MissingEqual => "an equals sign (=) was missing between a parameter and its value" , ParseError :: MissingQuote => "a quote (\") was missing from a parameter value" , ParseError :: InvalidToken { .. } => "invalid token" , ParseError :: InvalidRange => "unexpected asterisk" , ParseError :: TooLong => "the string is too long" , } ; if let ParseError :: InvalidToken { pos , byte } = * self { write ! (f , "{}, {:?} at position {}" , description , byte , pos) } else { f . write_str (description) } } }
};
}
