// Generated macro for test (module)
macro_rules! Depcrate_strategy_filter_maptest {
() => {
// Module: crate::strategy::filter_map
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_filter_map () { let input = (0 .. 256) . prop_filter_map ("%3 + 1" , | v | { if 0 == v % 3 { Some (v + 1) } else { None } }) ; for _ in 0 .. 256 { let mut runner = TestRunner :: default () ; let mut case = input . new_tree (& mut runner) . unwrap () ; assert_eq ! (0 , (case . current () - 1) % 3) ; while case . simplify () { assert_eq ! (0 , (case . current () - 1) % 3) ; } assert_eq ! (0 , (case . current () - 1) % 3) ; } } # [test] fn test_filter_map_sanity () { check_strategy_sanity ((0 .. 256) . prop_filter_map ("!%5 * 2" , | v | { if 0 != v % 5 { Some (v * 2) } else { None } }) , Some (CheckStrategySanityOptions { strict_complicate_after_simplify : false , .. CheckStrategySanityOptions :: default () }) ,) ; } }
};
}
