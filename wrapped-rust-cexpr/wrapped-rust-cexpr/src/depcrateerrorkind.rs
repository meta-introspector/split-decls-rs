// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Parsing errors specific to C parsing"] # [derive (Debug)] pub enum ErrorKind { # [doc = " Expected the specified token"] ExactToken (token :: Kind , & 'static [u8]) , # [doc = " Expected one of the specified tokens"] ExactTokens (token :: Kind , & 'static [& 'static str]) , # [doc = " Expected a token of the specified kind"] TypedToken (token :: Kind) , # [doc = " An unknown identifier was encountered"] UnknownIdentifier , # [doc = " An invalid literal was encountered."] # [doc = ""] # [doc = " When encountered, this generally means a bug exists in the data that"] # [doc = " was passed in or the parsing logic."] InvalidLiteral , # [doc = " A full parse was requested, but data was left over after parsing finished."] Partial , # [doc = " An error occurred in an underlying nom parser."] Parser (nom :: ErrorKind) , }
};
}
