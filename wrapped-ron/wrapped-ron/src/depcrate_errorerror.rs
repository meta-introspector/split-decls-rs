// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum Error { Fmt , Io (String) , Message (String) , Eof , ExpectedArray , ExpectedArrayEnd , ExpectedAttribute , ExpectedAttributeEnd , ExpectedBoolean , ExpectedComma , ExpectedChar , ExpectedByteLiteral , ExpectedFloat , FloatUnderscore , ExpectedInteger , ExpectedOption , ExpectedOptionEnd , ExpectedMap , ExpectedMapColon , ExpectedMapEnd , ExpectedDifferentStructName { expected : & 'static str , found : String , } , ExpectedStructLike , ExpectedNamedStructLike (& 'static str) , ExpectedStructLikeEnd , ExpectedUnit , ExpectedString , ExpectedByteString , ExpectedStringEnd , ExpectedIdentifier , InvalidEscape (& 'static str) , IntegerOutOfBounds , InvalidIntegerDigit { digit : char , base : u8 , } , NoSuchExtension (String) , UnclosedBlockComment , UnclosedLineComment , UnderscoreAtBeginning , UnexpectedChar (char) , Utf8Error (Utf8Error) , TrailingCharacters , InvalidValueForType { expected : String , found : String , } , ExpectedDifferentLength { expected : String , found : usize , } , NoSuchEnumVariant { expected : & 'static [& 'static str] , found : String , outer : Option < String > , } , NoSuchStructField { expected : & 'static [& 'static str] , found : String , outer : Option < String > , } , MissingStructField { field : & 'static str , outer : Option < String > , } , DuplicateStructField { field : & 'static str , outer : Option < String > , } , InvalidIdentifier (String) , SuggestRawIdentifier (String) , ExpectedRawValue , ExceededRecursionLimit , ExpectedStructName (String) , }
};
}
