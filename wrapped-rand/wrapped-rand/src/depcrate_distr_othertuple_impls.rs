// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_distr_othertuple_impls {
() => {
// Module: crate::distr::other
// Provides: {"tuple_impls"}
// Dependencies: {}
# [doc = " Looping wrapper for `tuple_impl`. Given (A, B, C), it also generates"] # [doc = " implementations for (A, B) and (A,)"] macro_rules ! tuple_impls { ($ ($ tyvar : ident) *) => { tuple_impls ! { [] $ ($ tyvar) * } } ; ([$ ($ prefix : ident) *] $ head : ident $ ($ tail : ident) *) => { tuple_impl ! { $ ($ prefix) * } tuple_impls ! { [$ ($ prefix) * $ head] $ ($ tail) * } } ; ([$ ($ prefix : ident) *]) => { tuple_impl ! { $ ($ prefix) * } } ; }
};
}
