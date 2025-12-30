// Generated macro for test_line (function)
macro_rules! Depcrate_teststest_line {
() => {
// Module: crate::tests
// Provides: {"test_line"}
// Dependencies: {}
# [test] fn test_line () { use text_size :: TextRange ; macro_rules ! validate { ($ text : expr , $ line : expr , $ expected_start : literal .. $ expected_end : literal) => { let line_index = LineIndex :: new ($ text) ; assert_eq ! (line_index . line ($ line) , Some (TextRange :: new (TextSize :: from ($ expected_start) , TextSize :: from ($ expected_end)))) ; } ; } validate ! ("" , 0 , 0 .. 0) ; validate ! ("\n" , 1 , 1 .. 1) ; validate ! ("\nabc" , 1 , 1 .. 4) ; validate ! ("\nabc\ndef" , 1 , 1 .. 5) ; }
};
}
