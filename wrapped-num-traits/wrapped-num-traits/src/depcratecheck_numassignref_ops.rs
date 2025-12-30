// Generated macro for check_numassignref_ops (function)
macro_rules! Depcratecheck_numassignref_ops {
() => {
// Module: crate
// Provides: {"check_numassignref_ops"}
// Dependencies: {}
# [test] fn check_numassignref_ops () { fn compute < T : NumAssignRef + Copy > (mut x : T , y : & T) -> T { x *= y ; x /= y ; x %= y ; x += y ; x -= y ; x } assert_eq ! (compute (1 , & 2) , 1) }
};
}
