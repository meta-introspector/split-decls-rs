// Generated macro for impl_22 (impl)
macro_rules! Depcrate_abs_diff_eqimpl_22 {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , T : AbsDiffEq + ? Sized > AbsDiffEq for & 'a mut T { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> T :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & & 'a mut T , epsilon : T :: Epsilon) -> bool { T :: abs_diff_eq (* self , * other , epsilon) } }
};
}
