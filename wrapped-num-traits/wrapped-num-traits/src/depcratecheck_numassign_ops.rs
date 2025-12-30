// Generated macro for check_numassign_ops (function)
macro_rules! Depcratecheck_numassign_ops {
() => {
// Module: crate
// Provides: {"check_numassign_ops"}
// Dependencies: {}
# [test] fn check_numassign_ops () { fn compute < T : NumAssign + Copy > (mut x : T , y : T) -> T { x *= y ; x /= y ; x %= y ; x += y ; x -= y ; x } assert_eq ! (compute (1 , 2) , 1) }
};
}
