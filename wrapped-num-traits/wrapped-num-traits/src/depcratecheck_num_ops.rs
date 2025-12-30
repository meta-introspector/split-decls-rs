// Generated macro for check_num_ops (function)
macro_rules! Depcratecheck_num_ops {
() => {
// Module: crate
// Provides: {"check_num_ops"}
// Dependencies: {}
# [test] fn check_num_ops () { fn compute < T : Num + Copy > (x : T , y : T) -> T { x * y / y % y + y - y } assert_eq ! (compute (1 , 2) , 1) }
};
}
