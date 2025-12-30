// Generated macro for test (module)
macro_rules! Depcrate_tupletest {
() => {
// Module: crate::tuple
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: strategy :: * ; use super :: * ; # [test] fn shrinks_fully_ltr () { fn pass (a : (i32 , i32)) -> bool { a . 0 * a . 1 <= 9 } let input = (0 .. 32 , 0 .. 32) ; let mut runner = TestRunner :: default () ; let mut cases_tested = 0 ; for _ in 0 .. 256 { let mut case = input . new_tree (& mut runner) . unwrap () ; if pass (case . current ()) { continue ; } loop { if pass (case . current ()) { if ! case . complicate () { break ; } } else { if ! case . simplify () { break ; } } } let last = case . current () ; assert ! (! pass (last)) ; assert ! (pass ((last . 0 - 1 , last . 1))) ; assert ! (pass ((last . 0 , last . 1 - 1))) ; cases_tested += 1 ; } assert ! (cases_tested > 32 , "Didn't find enough test cases") ; } # [test] fn test_sanity () { check_strategy_sanity ((0i32 .. 100 , 0i32 .. 1000 , 0i32 .. 10000) , None) ; } }
};
}
