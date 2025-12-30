// Generated macro for test_cpt_builder (function)
macro_rules! Depcratetest_cpt_builder {
() => {
// Module: crate
// Provides: {"test_cpt_builder"}
// Dependencies: {}
# [test] # [cfg (any (feature = "wasm" , feature = "icu4c"))] fn test_cpt_builder () { let values : Vec < u32 > = (0 .. 100) . map (| x | x / 10) . chain ((100 .. 0x100) . map (| _ | 100)) . chain ((0x100 .. 0x200) . map (| x | x % 16)) . collect () ; let builder = CodePointTrieBuilder { data : CodePointTrieBuilderData :: ValuesByCodePoint (& values) , default_value : 100 , error_value : 0xFFFF , trie_type : TrieType :: Fast , } ; let cpt = builder . build () ; assert_eq ! (cpt . get32 (0) , 0) ; assert_eq ! (cpt . get32 (10) , 1) ; assert_eq ! (cpt . get32 (20) , 2) ; assert_eq ! (cpt . get32 (21) , 2) ; assert_eq ! (cpt . get32 (99) , 9) ; assert_eq ! (cpt . get32 (0x101) , 0x1) ; assert_eq ! (cpt . get32 (0x102) , 0x2) ; assert_eq ! (cpt . get32 (0x105) , 0x5) ; assert_eq ! (cpt . get32 (0x125) , 0x5) ; assert_eq ! (cpt . get32 (0x135) , 0x5) ; assert_eq ! (cpt . get32 (0x13F) , 0xF) ; assert_eq ! (cpt . get32 (0x300) , 100) ; }
};
}
