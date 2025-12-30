// Generated macro for CodePointTrieBuilderData (enum)
macro_rules! DepcrateCodePointTrieBuilderData {
() => {
// Module: crate
// Provides: {"CodePointTrieBuilderData"}
// Dependencies: {}
# [doc = " Wrapper over the data to be encoded into a [`CodePointTrie`]."] # [doc = ""] # [doc = " There is currently only one variant, but more may be added in the future."] # [doc = ""] # [doc = " [`CodePointTrie`]: icu_collections::codepointtrie::CodePointTrie"] # [non_exhaustive] # [derive (Debug)] pub enum CodePointTrieBuilderData < 'a , T > { # [doc = " A list of values for each code point, starting from code point 0."] # [doc = ""] # [doc = " For example, the value for U+0020 (space) should be at index 32 in the slice."] # [doc = " Index 0 sets the value for the U+0000 (NUL)."] ValuesByCodePoint (& 'a [T]) , }
};
}
