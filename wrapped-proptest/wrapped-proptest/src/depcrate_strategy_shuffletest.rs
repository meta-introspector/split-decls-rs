// Generated macro for test (module)
macro_rules! Depcrate_strategy_shuffletest {
() => {
// Module: crate::strategy::shuffle
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use std :: borrow :: ToOwned ; use std :: collections :: HashSet ; use std :: i32 ; use super :: * ; use crate :: collection ; use crate :: strategy :: just :: Just ; static VALUES : & 'static [i32] = & [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 ,] ; # [test] fn generates_different_permutations () { let mut runner = TestRunner :: default () ; let mut seen = HashSet :: < Vec < i32 > > :: new () ; let input = Just (VALUES . to_owned ()) . prop_shuffle () ; for _ in 0 .. 1024 { let mut value = input . new_tree (& mut runner) . unwrap () . current () ; assert ! (seen . insert (value . clone ()) , "Value {:?} generated more than once" , value) ; value . sort () ; assert_eq ! (VALUES , & value [..]) ; } } # [test] fn simplify_reduces_shuffle_amount () { let mut runner = TestRunner :: default () ; let input = Just (VALUES . to_owned ()) . prop_shuffle () ; for _ in 0 .. 1024 { let mut value = input . new_tree (& mut runner) . unwrap () ; let mut prev_dist = i32 :: MAX ; loop { let v = value . current () ; let mut dist = 0 ; for (ix , & nominal) in v . iter () . enumerate () { dist += (nominal - ix as i32) . abs () ; } assert ! (dist <= prev_dist , "dist = {}, prev_dist = {}" , dist , prev_dist) ; prev_dist = dist ; if ! value . simplify () { break ; } } assert_eq ! (0 , prev_dist) ; } } # [test] fn simplify_complicate_contract_upheld () { check_strategy_sanity (collection :: vec (0i32 .. 1000 , 5 .. 10) . prop_shuffle () , None ,) ; } }
};
}
