// Generated macro for ParsedMatchPattern (struct)
macro_rules! Depcrate_matches_patternParsedMatchPattern {
() => {
// Module: crate::matches_pattern
// Provides: {"ParsedMatchPattern"}
// Dependencies: {}
# [doc = " Represents one of the root-level pseudo-patterns supported by"] # [doc = " `match_pattern!`."] # [doc = ""] # [doc = " Examples with `struct_name` set and `group` being `None`:"] # [doc = " * `true`"] # [doc = " * `Enum::Variant`"] # [doc = " * `&Enum::Variant`"] # [doc = ""] # [doc = " Examples with `group` being non-`None`."] # [doc = " * `Struct { a: eq(1), b: ends_with(\"foo\") }`"] # [doc = " * `&Struct(ref eq(&1), ends_with(\"foo\"))`"] # [derive (Debug)] struct ParsedMatchPattern { struct_name : TokenStream , group : Option < Group > , }
};
}
