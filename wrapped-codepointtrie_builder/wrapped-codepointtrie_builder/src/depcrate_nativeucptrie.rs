// Generated macro for UCPTrie (struct)
macro_rules! Depcrate_nativeUCPTrie {
() => {
// Module: crate::native
// Provides: {"UCPTrie"}
// Dependencies: {}
# [repr (C)] # [allow (non_snake_case)] pub struct UCPTrie { pub index : * const u16 , pub data : UCPTrieData , pub indexLength : i32 , pub dataLength : i32 , pub highStart : u32 , pub shifted12HighStart : u16 , pub type_ : i8 , pub valueWidth : i8 , pub reserved32 : u32 , pub reserved16 : u16 , pub index3NullOffset : u16 , pub dataNullOffset : i32 , pub nullValue : u32 , }
};
}
