// Generated macro for is_word_char (module)
macro_rules! Depcrate_util_lookis_word_char {
() => {
// Module: crate::util::look
// Provides: {"is_word_char"}
// Dependencies: {}
# [doc = " A module that always returns an error if Unicode word boundaries are"] # [doc = " disabled. When this feature is disabled, then regex-automata will not"] # [doc = " include its own data tables even if regex-syntax is disabled."] # [cfg (not (feature = "unicode-word-boundary"))] mod is_word_char { pub (super) fn check () -> Result < () , super :: UnicodeWordBoundaryError > { Err (super :: UnicodeWordBoundaryError :: new ()) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (super) fn fwd (_bytes : & [u8] , _at : usize ,) -> Result < bool , super :: UnicodeWordBoundaryError > { Err (super :: UnicodeWordBoundaryError :: new ()) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (super) fn rev (_bytes : & [u8] , _at : usize ,) -> Result < bool , super :: UnicodeWordBoundaryError > { Err (super :: UnicodeWordBoundaryError :: new ()) } }
};
}
