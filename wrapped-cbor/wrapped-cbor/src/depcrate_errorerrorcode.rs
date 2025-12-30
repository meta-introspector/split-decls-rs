// Generated macro for ErrorCode (enum)
macro_rules! Depcrate_errorErrorCode {
() => {
// Module: crate::error
// Provides: {"ErrorCode"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum ErrorCode { # [cfg (feature = "std")] Message (String) , # [cfg (not (feature = "std"))] Message , # [cfg (feature = "std")] Io (io :: Error) , # [allow (unused)] # [cfg (not (feature = "std"))] Io , ScratchTooSmall , EofWhileParsingValue , EofWhileParsingArray , EofWhileParsingMap , LengthOutOfRange , InvalidUtf8 , UnassignedCode , UnexpectedCode , TrailingData , ArrayTooShort , ArrayTooLong , RecursionLimitExceeded , WrongEnumFormat , WrongStructFormat , }
};
}
