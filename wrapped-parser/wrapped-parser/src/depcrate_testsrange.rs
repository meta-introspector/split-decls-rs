// Generated macro for range (function)
macro_rules! Depcrate_testsrange {
() => {
// Module: crate::tests
// Provides: {"range"}
// Dependencies: {}
# [rstest] # [case ("{=0..4}" , 0 .. 4)] # [case :: just_inside_128bit_range_1 ("{=0..128}" , 0 .. 128)] # [case :: just_inside_128bit_range_2 ("{=127..128}" , 127 .. 128)] fn range (# [case] input : & str , # [case] bit_field : Range < u8 >) { assert_eq ! (parse (input , ParserMode :: Strict) , Ok (vec ! [Fragment :: Parameter (Parameter { index : 0 , ty : Type :: BitField (bit_field) , hint : None , })])) ; }
};
}
