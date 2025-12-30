// Generated macro for ParsedAnnotations (struct)
macro_rules! Depcrate_fmt_rfc9557ParsedAnnotations {
() => {
// Module: crate::fmt::rfc9557
// Provides: {"ParsedAnnotations"}
// Dependencies: {}
# [doc = " The result of parsing RFC 9557 annotations."] # [doc = ""] # [doc = " Currently, this only provides access to a parsed time zone annotation, if"] # [doc = " present. While the parser does validate all other key/value annotations,"] # [doc = " Jiff doesn't make use of them and thus does not expose them here. They are"] # [doc = " only validated at a syntax level."] # [derive (Debug)] pub (crate) struct ParsedAnnotations < 'i > { # [doc = " The original input that all of the annotations were parsed from."] # [doc = ""] # [doc = " N.B. This is currently unused, but potentially useful, so we leave it."] # [allow (dead_code)] input : escape :: Bytes < 'i > , # [doc = " An optional time zone annotation that was extracted from the input."] time_zone : Option < ParsedTimeZone < 'i > > , }
};
}
