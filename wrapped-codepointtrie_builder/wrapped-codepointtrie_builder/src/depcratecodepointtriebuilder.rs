// Generated macro for CodePointTrieBuilder (struct)
macro_rules! DepcrateCodePointTrieBuilder {
() => {
// Module: crate
// Provides: {"CodePointTrieBuilder"}
// Dependencies: {}
# [doc = " Settings for building a [`CodePointTrie`]."] # [doc = ""] # [doc = " [`CodePointTrie`]: icu_collections::codepointtrie::CodePointTrie"] # [allow (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct CodePointTrieBuilder < 'a , T > { # [doc = " The data to be encoded."] pub data : CodePointTrieBuilderData < 'a , T > , # [doc = " The default value for code points not specified in the data."] pub default_value : T , # [doc = " The error value for invalid code points."] pub error_value : T , # [doc = " The [`TrieType`]: fast or small."] pub trie_type : TrieType , }
};
}
