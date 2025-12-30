// Generated macro for Sunk (struct)
macro_rules! Depcrate_utilSunk {
() => {
// Module: crate::util
// Provides: {"Sunk"}
// Dependencies: {}
# [doc = " A simple layer of abstraction over either a match or a contextual line"] # [doc = " reported by the searcher."] # [doc = ""] # [doc = " In particular, this provides an API that unions the `SinkMatch` and"] # [doc = " `SinkContext` types while also exposing a list of all individual match"] # [doc = " locations."] # [doc = ""] # [doc = " While this serves as a convenient mechanism to abstract over `SinkMatch`"] # [doc = " and `SinkContext`, this also provides a way to abstract over replacements."] # [doc = " Namely, after a replacement, a `Sunk` value can be constructed using the"] # [doc = " results of the replacement instead of the bytes reported directly by the"] # [doc = " searcher."] # [derive (Debug)] pub (crate) struct Sunk < 'a > { bytes : & 'a [u8] , absolute_byte_offset : u64 , line_number : Option < u64 > , context_kind : Option < & 'a SinkContextKind > , matches : & 'a [Match] , original_matches : & 'a [Match] , }
};
}
