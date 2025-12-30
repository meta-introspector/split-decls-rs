// Generated macro for utf8_error_description (function)
macro_rules! Depcrate_errorsutf8_error_description {
() => {
// Module: crate::errors
// Provides: {"utf8_error_description"}
// Dependencies: {}
const fn utf8_error_description (kind : Utf8ErrorKind) -> & 'static str { match kind { Utf8ErrorKind :: TooFewBytes => "too few bytes" , Utf8ErrorKind :: NonUtf8Byte => "not UTF-8" , Utf8ErrorKind :: UnexpectedContinuationByte => "not UTF-8" , Utf8ErrorKind :: InterruptedSequence => "not UTF-8" , Utf8ErrorKind :: OverlongEncoding => "malformed input" , Utf8ErrorKind :: Utf16ReservedCodepoint => "malformed input" , Utf8ErrorKind :: TooHighCodepoint => "invalid character" , } }
};
}
