// Generated macro for impl_36 (impl)
macro_rules! Depcrate_humanimpl_36 {
() => {
// Module: crate::human
// Provides: {"impl_36"}
// Dependencies: {}
impl ParseSizeError { fn format (original : & str) -> ParseSizeError { ParseSizeError { original : original . to_string () , kind : ParseSizeErrorKind :: InvalidFormat , } } fn int (original : & str , err : std :: num :: ParseIntError) -> ParseSizeError { ParseSizeError { original : original . to_string () , kind : ParseSizeErrorKind :: InvalidInt (err) , } } fn overflow (original : & str) -> ParseSizeError { ParseSizeError { original : original . to_string () , kind : ParseSizeErrorKind :: Overflow , } } }
};
}
