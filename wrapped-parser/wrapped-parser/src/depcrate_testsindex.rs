// Generated macro for index (function)
macro_rules! Depcrate_testsindex {
() => {
// Module: crate::tests
// Provides: {"index"}
// Dependencies: {}
# [rstest] # [case :: implicit ("{=u8}{=u16}" , [(0 , Type :: U8) , (1 , Type :: U16)])] # [case :: single_parameter_formatted_twice ("{=u8}{0=u8}" , [(0 , Type :: U8) , (0 , Type :: U8)])] # [case :: explicit_index ("{=u8}{1=u16}" , [(0 , Type :: U8) , (1 , Type :: U16)])] # [case :: reversed_order ("{1=u8}{0=u16}" , [(1 , Type :: U8) , (0 , Type :: U16)])] fn index (# [case] input : & str , # [case] params : [(usize , Type) ; 2]) { assert_eq ! (parse (input , ParserMode :: Strict) , Ok (vec ! [Fragment :: Parameter (Parameter { index : params [0] . 0 , ty : params [0] . 1 . clone () , hint : None , }) , Fragment :: Parameter (Parameter { index : params [1] . 0 , ty : params [1] . 1 . clone () , hint : None , }) ,])) ; }
};
}
