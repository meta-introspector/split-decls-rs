// Generated macro for test (module)
macro_rules! Depcrate_optiontest {
() => {
// Module: crate::option
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; fn count_some_of_1000 (s : OptionStrategy < Just < i32 > >) -> u32 { let mut runner = TestRunner :: deterministic () ; let mut count = 0 ; for _ in 0 .. 1000 { count += s . new_tree (& mut runner) . unwrap () . current () . is_some () as u32 ; } count } # [test] fn probability_defaults_to_0p5 () { let count = count_some_of_1000 (of (Just (42i32))) ; assert ! (count > 450 && count < 550) ; } # [test] fn probability_handled_correctly () { let count = count_some_of_1000 (weighted (0.9 , Just (42i32))) ; assert ! (count > 800 && count < 950) ; let count = count_some_of_1000 (weighted (0.1 , Just (42i32))) ; assert ! (count > 50 && count < 150) ; } # [test] fn test_sanity () { check_strategy_sanity (of (0i32 .. 1000i32) , None) ; } }
};
}
