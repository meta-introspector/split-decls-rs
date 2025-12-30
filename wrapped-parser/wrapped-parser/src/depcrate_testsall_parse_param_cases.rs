// Generated macro for all_parse_param_cases (function)
macro_rules! Depcrate_testsall_parse_param_cases {
() => {
// Module: crate::tests
// Provides: {"all_parse_param_cases"}
// Dependencies: {}
# [rstest] # [case :: noo_param ("" , None , Type :: Format , None)] # [case :: one_param_type ("=u8" , None , Type :: U8 , None)] # [case :: one_param_hint (":a" , None , Type :: Format , Some (DisplayHint :: Ascii))] # [case :: one_param_index ("1" , Some (1) , Type :: Format , None)] # [case :: two_param_type_hint ("=u8:x" , None , Type :: U8 , Some (DisplayHint :: Hexadecimal { alternate : false , uppercase : false , zero_pad : 0 }))] # [case :: two_param_index_type ("0=u8" , Some (0) , Type :: U8 , None)] # [case :: two_param_index_hint ("0:a" , Some (0) , Type :: Format , Some (DisplayHint :: Ascii))] # [case :: two_param_type_hint ("=[u8]:#04x" , None , Type :: U8Slice , Some (DisplayHint :: Hexadecimal { alternate : true , uppercase : false , zero_pad : 4 }))] # [case :: all_param ("1=u8:b" , Some (1) , Type :: U8 , Some (DisplayHint :: Binary { alternate : false , zero_pad : 0 }))] fn all_parse_param_cases (# [case] input : & str , # [case] index : Option < usize > , # [case] ty : Type , # [case] hint : Option < DisplayHint > ,) { assert_eq ! (parse_param (input , ParserMode :: Strict) , Ok (Param { index , ty , hint })) ; }
};
}
