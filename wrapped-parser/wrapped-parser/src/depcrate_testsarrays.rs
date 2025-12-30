// Generated macro for arrays (function)
macro_rules! Depcrate_testsarrays {
() => {
// Module: crate::tests
// Provides: {"arrays"}
// Dependencies: {}
# [rstest] # [case ("{=[u8; 0]}" , 0)] # [case :: space_is_optional ("{=[u8;42]}" , 42)] # [case :: multiple_spaces_are_ok ("{=[u8;    257]}" , 257)] fn arrays (# [case] input : & str , # [case] length : usize) { assert_eq ! (parse (input , ParserMode :: Strict) , Ok (vec ! [Fragment :: Parameter (Parameter { index : 0 , ty : Type :: U8Array (length) , hint : None , })])) ; }
};
}
