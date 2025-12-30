// Generated macro for test (module)
macro_rules! Depcrate_strategy_filtertest {
() => {
// Module: crate::strategy::filter
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_filter () { let input = (0 .. 256) . prop_filter ("%3" , | & v | 0 == v % 3) ; for _ in 0 .. 256 { let mut runner = TestRunner :: default () ; let mut case = input . new_tree (& mut runner) . unwrap () ; assert ! (0 == case . current () % 3) ; while case . simplify () { assert ! (0 == case . current () % 3) ; } assert ! (0 == case . current () % 3) ; } } # [test] fn test_filter_sanity () { check_strategy_sanity ((0 .. 256) . prop_filter ("!%5" , | & v | 0 != v % 5) , Some (CheckStrategySanityOptions { strict_complicate_after_simplify : false , .. CheckStrategySanityOptions :: default () }) ,) ; } }
};
}
