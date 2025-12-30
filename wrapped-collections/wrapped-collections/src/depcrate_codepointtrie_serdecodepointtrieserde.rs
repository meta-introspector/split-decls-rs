// Generated macro for CodePointTrieSerde (struct)
macro_rules! Depcrate_codepointtrie_serdeCodePointTrieSerde {
() => {
// Module: crate::codepointtrie::serde
// Provides: {"CodePointTrieSerde"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] pub struct CodePointTrieSerde < 'trie , T : TrieValue > { header : CodePointTrieHeader , # [serde (borrow)] index : ZeroVec < 'trie , u16 > , # [serde (borrow)] data : ZeroVec < 'trie , T > , }
};
}
