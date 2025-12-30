// Generated macro for Flags (struct)
macro_rules! Depcrate_dfa_denseFlags {
() => {
// Module: crate::dfa::dense
// Provides: {"Flags"}
// Dependencies: {}
# [doc = " A common set of flags for both dense and sparse DFAs. This primarily"] # [doc = " centralizes the serialization format of these flags at a bitset."] # [derive (Clone , Copy , Debug)] pub (crate) struct Flags { # [doc = " Whether the DFA can match the empty string. When this is false, all"] # [doc = " matches returned by this DFA are guaranteed to have non-zero length."] pub (crate) has_empty : bool , # [doc = " Whether the DFA should only produce matches with spans that correspond"] # [doc = " to valid UTF-8. This also includes omitting any zero-width matches that"] # [doc = " split the UTF-8 encoding of a codepoint."] pub (crate) is_utf8 : bool , # [doc = " Whether the DFA is always anchored or not, regardless of `Input`"] # [doc = " configuration. This is useful for avoiding a reverse scan even when"] # [doc = " executing unanchored searches."] pub (crate) is_always_start_anchored : bool , }
};
}
