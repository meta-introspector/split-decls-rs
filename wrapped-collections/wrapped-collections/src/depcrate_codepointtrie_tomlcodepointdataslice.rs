// Generated macro for CodePointDataSlice (enum)
macro_rules! Depcrate_codepointtrie_tomlCodePointDataSlice {
() => {
// Module: crate::codepointtrie::toml
// Provides: {"CodePointDataSlice"}
// Dependencies: {}
# [doc = " Data slice from a [`CodePointTrie`] TOML."] # [doc = ""] # [doc = " ICU4C exports data as either `u8`, `u16`, or `u32`, which may be converted"] # [doc = " to other types as appropriate."] # [allow (clippy :: exhaustive_enums)] pub enum CodePointDataSlice < 'a > { # [doc = " A serialized [`CodePointTrie`] data array 8-bit values."] U8 (& 'a [u8]) , # [doc = " A serialized [`CodePointTrie`] data array 16-bit values."] U16 (& 'a [u16]) , # [doc = " A serialized [`CodePointTrie`] data array 32-bit values."] U32 (& 'a [u32]) , }
};
}
