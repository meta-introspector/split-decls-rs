// Generated macro for multiple_rules (function)
macro_rules! Depcrate_testsmultiple_rules {
() => {
// Module: crate::tests
// Provides: {"multiple_rules"}
// Dependencies: {}
# [test] fn multiple_rules () { assert_ssr_transforms (& ["$a + 1 ==>> add_one($a)" , "$a + $b ==>> add($a, $b)"] , "fn add() {} fn add_one() {} fn f() -> i32 {3 + 2 + 1}" , expect ! [["fn add() {} fn add_one() {} fn f() -> i32 {add_one(add(3, 2))}"]] ,) }
};
}
