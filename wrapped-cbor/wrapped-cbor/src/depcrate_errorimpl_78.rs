// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
impl fmt :: Display for ErrorCode { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { # [cfg (feature = "std")] ErrorCode :: Message (ref msg) => f . write_str (msg) , # [cfg (not (feature = "std"))] ErrorCode :: Message => f . write_str ("Unknown error") , # [cfg (feature = "std")] ErrorCode :: Io (ref err) => fmt :: Display :: fmt (err , f) , # [cfg (not (feature = "std"))] ErrorCode :: Io => f . write_str ("Unknown I/O error") , ErrorCode :: ScratchTooSmall => f . write_str ("Scratch buffer too small") , ErrorCode :: EofWhileParsingValue => f . write_str ("EOF while parsing a value") , ErrorCode :: EofWhileParsingArray => f . write_str ("EOF while parsing an array") , ErrorCode :: EofWhileParsingMap => f . write_str ("EOF while parsing a map") , ErrorCode :: LengthOutOfRange => f . write_str ("length out of range") , ErrorCode :: InvalidUtf8 => f . write_str ("invalid UTF-8") , ErrorCode :: UnassignedCode => f . write_str ("unassigned type") , ErrorCode :: UnexpectedCode => f . write_str ("unexpected code") , ErrorCode :: TrailingData => f . write_str ("trailing data") , ErrorCode :: ArrayTooShort => f . write_str ("array too short") , ErrorCode :: ArrayTooLong => f . write_str ("array too long") , ErrorCode :: RecursionLimitExceeded => f . write_str ("recursion limit exceeded") , ErrorCode :: WrongEnumFormat => f . write_str ("wrong enum format") , ErrorCode :: WrongStructFormat => f . write_str ("wrong struct format") , } } }
};
}
