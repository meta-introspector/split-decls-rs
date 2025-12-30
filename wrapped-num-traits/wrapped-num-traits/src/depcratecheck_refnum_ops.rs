// Generated macro for check_refnum_ops (function)
macro_rules! Depcratecheck_refnum_ops {
() => {
// Module: crate
// Provides: {"check_refnum_ops"}
// Dependencies: {}
# [test] fn check_refnum_ops () { fn compute < T : Copy > (x : & T , y : T) -> T where for < 'a > & 'a T : RefNum < T > , { & (& (& (& (x * y) / y) % y) + y) - y } assert_eq ! (compute (& 1 , 2) , 1) }
};
}
