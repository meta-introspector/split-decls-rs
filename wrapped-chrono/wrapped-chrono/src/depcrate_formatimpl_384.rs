// Generated macro for impl_384 (impl)
macro_rules! Depcrate_formatimpl_384 {
() => {
// Module: crate::format
// Provides: {"impl_384"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ParseErrorKind :: OutOfRange => write ! (f , "input is out of range") , ParseErrorKind :: Impossible => write ! (f , "no possible date and time matching input") , ParseErrorKind :: NotEnough => write ! (f , "input is not enough for unique date and time") , ParseErrorKind :: Invalid => write ! (f , "input contains invalid characters") , ParseErrorKind :: TooShort => write ! (f , "premature end of input") , ParseErrorKind :: TooLong => write ! (f , "trailing input") , ParseErrorKind :: BadFormat => write ! (f , "bad or unsupported format string") , _ => unreachable ! () , } } }
};
}
