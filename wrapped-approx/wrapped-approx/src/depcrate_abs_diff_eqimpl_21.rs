// Generated macro for impl_21 (impl)
macro_rules! Depcrate_abs_diff_eqimpl_21 {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , T : AbsDiffEq + ? Sized > AbsDiffEq for & 'a T { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> T :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & & 'a T , epsilon : T :: Epsilon) -> bool { T :: abs_diff_eq (* self , * other , epsilon) } }
};
}
