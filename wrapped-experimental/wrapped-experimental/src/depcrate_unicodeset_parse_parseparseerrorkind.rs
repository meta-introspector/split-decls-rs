// Generated macro for ParseErrorKind (enum)
macro_rules! Depcrate_unicodeset_parse_parseParseErrorKind {
() => {
// Module: crate::unicodeset_parse::parse
// Provides: {"ParseErrorKind"}
// Dependencies: {}
# [doc = " The kind of error that occurred."] # [derive (Debug , Clone , Copy , PartialEq , Eq , displaydoc :: Display)] # [non_exhaustive] pub enum ParseErrorKind { # [doc = " An unexpected character was encountered."] # [doc = ""] # [doc = " This variant implies the other variants"] # [doc = " (notably `UnknownProperty` and `Unimplemented`) do not apply."] # [displaydoc ("An unexpected character was encountered")] UnexpectedChar (char) , # [doc = " The property name or value is unknown."] # [doc = ""] # [doc = " For property names, make sure you use the spelling"] # [doc = " defined in [ECMA-262](https://tc39.es/ecma262/#table-nonbinary-unicode-properties)."] # [displaydoc ("The property name or value is unknown")] UnknownProperty , # [doc = " A reference to an unknown variable."] UnknownVariable , # [doc = " A variable of a certain type occurring in an unexpected context."] UnexpectedVariable , # [doc = " The source is an incomplete unicode set."] Eof , # [doc = " Something unexpected went wrong with our code. Please file a bug report on GitHub."] Internal , # [doc = " The provided syntax is not supported by us."] # [doc = ""] # [doc = " Note that unknown properties will return the"] # [doc = " `UnknownProperty` variant, not this one."] # [displaydoc ("The provided syntax is not supported by us.")] Unimplemented , # [doc = " The provided escape sequence is not a valid Unicode code point or represents too many code points."] InvalidEscape , }
};
}
