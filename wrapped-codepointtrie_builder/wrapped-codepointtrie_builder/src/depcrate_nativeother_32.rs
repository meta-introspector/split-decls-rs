// Generated macro for other_32 (other)
macro_rules! Depcrate_nativeother_32 {
() => {
// Module: crate::native
// Provides: {"other_32"}
// Dependencies: {}
extern "C" { # [cfg_attr (icu4c_enable_renaming , link_name = concat ! ("umutablecptrie_open_" , env ! ("ICU4C_RENAME_VERSION")))] fn umutablecptrie_open (initial_value : u32 , error_value : u32 , error_code : & mut u32 ,) -> * const UMutableCPTrie ; # [cfg_attr (icu4c_enable_renaming , link_name = concat ! ("umutablecptrie_set_" , env ! ("ICU4C_RENAME_VERSION")))] fn umutablecptrie_set (trie : * const UMutableCPTrie , cp : u32 , value : u32 , error_code : & mut u32 ,) -> * const UMutableCPTrie ; # [cfg_attr (icu4c_enable_renaming , link_name = concat ! ("umutablecptrie_buildImmutable_" , env ! ("ICU4C_RENAME_VERSION")))] fn umutablecptrie_buildImmutable (trie : * const UMutableCPTrie , trie_type : u32 , width : u32 , error_code : & mut u32 ,) -> * const UCPTrie ; # [cfg_attr (icu4c_enable_renaming , link_name = concat ! ("ucptrie_close_" , env ! ("ICU4C_RENAME_VERSION")))] fn ucptrie_close (trie : * const UCPTrie) ; # [cfg_attr (icu4c_enable_renaming , link_name = concat ! ("umutablecptrie_close_" , env ! ("ICU4C_RENAME_VERSION")))] fn umutablecptrie_close (builder : * const UMutableCPTrie) ; }
};
}
