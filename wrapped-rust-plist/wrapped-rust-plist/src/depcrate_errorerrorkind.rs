// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum ErrorKind { UnexpectedEof , UnexpectedEndOfEventStream , UnexpectedEventType { # [allow (dead_code)] expected : EventKind , # [allow (dead_code)] found : EventKind , } , ExpectedEndOfEventStream { # [allow (dead_code)] found : EventKind , } , UnclosedString , IncompleteComment , InvalidUtf8AsciiStream , InvalidOctalString , UnclosedXmlElement , UnexpectedXmlCharactersExpectedElement , UnexpectedXmlOpeningTag , UnknownXmlElement , InvalidXmlSyntax , InvalidXmlUtf8 , InvalidDataString , InvalidDateString , InvalidIntegerString , InvalidRealString , UidNotSupportedInXmlPlist , ObjectTooLarge , InvalidMagic , InvalidTrailerObjectOffsetSize , InvalidTrailerObjectReferenceSize , InvalidObjectLength , ObjectReferenceTooLarge , ObjectOffsetTooLarge , RecursiveObject , NullObjectUnimplemented , FillObjectUnimplemented , IntegerOutOfRange , InfiniteOrNanDate , InvalidUtf8String , InvalidUtf16String , UnknownObjectType (# [allow (dead_code)] u8 ,) , Io (io :: Error) , # [cfg (feature = "serde")] Serde (# [allow (dead_code)] String ,) , }
};
}
