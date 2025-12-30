// Generated macro for min_by_key (function)
macro_rules! Depcrate_testmin_by_key {
() => {
// Module: crate::test
// Provides: {"min_by_key"}
// Dependencies: {}
# [test] fn min_by_key () { let it = convert (vec ! [0 , 3 , 1 , - 10] . into_iter () . map (Ok :: < i32 , i32 >)) ; assert_eq ! (it . clone () . min_by_key (|& i | Ok (- i)) , Ok (Some (3))) ; assert_eq ! (it . clone () . min_by_key (|& i | Err ::< i32 , _ > (i)) , Err (0)) ; assert_eq ! (it . min_by_key (|& i | if i > 0 { Err (i) } else { Ok (- i) }) , Err (3)) ; }
};
}
