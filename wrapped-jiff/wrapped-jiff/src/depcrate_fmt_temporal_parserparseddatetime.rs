// Generated macro for ParsedDateTime (struct)
macro_rules! Depcrate_fmt_temporal_parserParsedDateTime {
() => {
// Module: crate::fmt::temporal::parser
// Provides: {"ParsedDateTime"}
// Dependencies: {}
# [doc = " The datetime components parsed from a string."] # [derive (Debug)] pub (super) struct ParsedDateTime < 'i > { # [doc = " The original input that the datetime was parsed from."] input : escape :: Bytes < 'i > , # [doc = " A required civil date."] date : ParsedDate < 'i > , # [doc = " An optional civil time."] time : Option < ParsedTime < 'i > > , # [doc = " An optional UTC offset."] offset : Option < ParsedOffset > , # [doc = " An optional RFC 9557 annotations parsed."] # [doc = ""] # [doc = " An empty `ParsedAnnotations` is valid and possible, so this bakes"] # [doc = " optionality into the type and doesn't need to be an `Option` itself."] annotations : ParsedAnnotations < 'i > , }
};
}
