// Generated macro for check_refref_ops (function)
macro_rules! Depcratecheck_refref_ops {
() => {
// Module: crate
// Provides: {"check_refref_ops"}
// Dependencies: {}
# [test] fn check_refref_ops () { fn compute < T > (x : & T , y : & T) -> T where for < 'a > & 'a T : RefNum < T > , { & (& (& (& (x * y) / y) % y) + y) - y } assert_eq ! (compute (& 1 , & 2) , 1) }
};
}
