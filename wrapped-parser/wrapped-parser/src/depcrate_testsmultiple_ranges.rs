// Generated macro for multiple_ranges (function)
macro_rules! Depcrate_testsmultiple_ranges {
() => {
// Module: crate::tests
// Provides: {"multiple_ranges"}
// Dependencies: {}
# [test] fn multiple_ranges () { assert_eq ! (parse ("{0=30..31}{1=0..4}{1=2..6}" , ParserMode :: Strict) , Ok (vec ! [Fragment :: Parameter (Parameter { index : 0 , ty : Type :: BitField (30 .. 31) , hint : None , }) , Fragment :: Parameter (Parameter { index : 1 , ty : Type :: BitField (0 .. 4) , hint : None , }) , Fragment :: Parameter (Parameter { index : 1 , ty : Type :: BitField (2 .. 6) , hint : None , }) ,])) ; }
};
}
