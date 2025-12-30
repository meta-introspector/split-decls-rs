// Generated macro for impl_24 (impl)
macro_rules! Depcrate_abs_diff_eqimpl_24 {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_24"}
// Dependencies: {}
impl < T : AbsDiffEq + ? Sized > AbsDiffEq for cell :: RefCell < T > { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> T :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & cell :: RefCell < T > , epsilon : T :: Epsilon) -> bool { T :: abs_diff_eq (& self . borrow () , & other . borrow () , epsilon) } }
};
}
