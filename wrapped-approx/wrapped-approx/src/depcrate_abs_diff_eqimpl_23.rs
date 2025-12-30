// Generated macro for impl_23 (impl)
macro_rules! Depcrate_abs_diff_eqimpl_23 {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : AbsDiffEq + Copy > AbsDiffEq for cell :: Cell < T > { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> T :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & cell :: Cell < T > , epsilon : T :: Epsilon) -> bool { T :: abs_diff_eq (& self . get () , & other . get () , epsilon) } }
};
}
