// Generated macro for impl_unsigned_abs_diff_eq (macro)
macro_rules! Depcrate_abs_diff_eqimpl_unsigned_abs_diff_eq {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_unsigned_abs_diff_eq"}
// Dependencies: {}
macro_rules ! impl_unsigned_abs_diff_eq { ($ T : ident , $ default_epsilon : expr) => { impl AbsDiffEq for $ T { type Epsilon = $ T ; # [inline] fn default_epsilon () -> $ T { $ default_epsilon } # [inline] fn abs_diff_eq (& self , other : &$ T , epsilon : $ T) -> bool { (if self > other { self - other } else { other - self }) <= epsilon } } } ; }
};
}
