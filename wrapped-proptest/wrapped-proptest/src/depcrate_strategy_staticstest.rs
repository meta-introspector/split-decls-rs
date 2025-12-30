// Generated macro for test (module)
macro_rules! Depcrate_strategy_staticstest {
() => {
// Module: crate::strategy::statics
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_static_filter () { # [derive (Clone , Copy , Debug)] struct MyFilter ; impl FilterFn < i32 > for MyFilter { fn apply (& self , & v : & i32) -> bool { 0 == v % 3 } } let input = Filter :: new (0 .. 256 , "%3" . into () , MyFilter) ; for _ in 0 .. 256 { let mut runner = TestRunner :: default () ; let mut case = input . new_tree (& mut runner) . unwrap () ; assert ! (0 == case . current () % 3) ; while case . simplify () { assert ! (0 == case . current () % 3) ; } assert ! (0 == case . current () % 3) ; } } # [test] fn test_static_map () { # [derive (Clone , Copy , Debug)] struct MyMap ; impl MapFn < i32 > for MyMap { type Output = i32 ; fn apply (& self , v : i32) -> i32 { v * 2 } } let input = Map :: new (0 .. 10 , MyMap) ; TestRunner :: default () . run (& input , | v | { assert ! (0 == v % 2) ; Ok (()) }) . unwrap () ; } }
};
}
