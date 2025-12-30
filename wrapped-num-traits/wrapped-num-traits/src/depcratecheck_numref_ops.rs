// Generated macro for check_numref_ops (function)
macro_rules! Depcratecheck_numref_ops {
() => {
// Module: crate
// Provides: {"check_numref_ops"}
// Dependencies: {}
# [test] fn check_numref_ops () { fn compute < T : NumRef > (x : T , y : & T) -> T { x * y / y % y + y - y } assert_eq ! (compute (1 , & 2) , 1) }
};
}
