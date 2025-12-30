// Generated macro for Error (enum)
macro_rules! Depcrate_parseError {
() => {
// Module: crate::parse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [parse()][crate::parse()]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("An empty string is not a valid pathspec")] EmptyString , # [error ("Found {keyword:?} in signature, which is not a valid keyword")] InvalidKeyword { keyword : BString } , # [error ("Unimplemented short keyword: {short_keyword:?}")] Unimplemented { short_keyword : char } , # [error ("Missing ')' at the end of pathspec signature")] MissingClosingParenthesis , # [error ("Attribute has non-ascii characters or starts with '-': {attribute:?}")] InvalidAttribute { attribute : BString } , # [error ("Invalid character in attribute value: {character:?}")] InvalidAttributeValue { character : char } , # [error (r"Escape character '\' is not allowed as the last character in an attribute value")] TrailingEscapeCharacter , # [error ("Attribute specification cannot be empty")] EmptyAttribute , # [error ("Only one attribute specification is allowed in the same pathspec")] MultipleAttributeSpecifications , # [error ("'literal' and 'glob' keywords cannot be used together in the same pathspec")] IncompatibleSearchModes , }
};
}
