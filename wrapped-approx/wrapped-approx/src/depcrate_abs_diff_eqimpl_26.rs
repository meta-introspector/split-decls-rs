// Generated macro for impl_26 (impl)
macro_rules! Depcrate_abs_diff_eqimpl_26 {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "num-complex")] impl < T : AbsDiffEq > AbsDiffEq for Complex < T > where T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> T :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Complex < T > , epsilon : T :: Epsilon) -> bool { T :: abs_diff_eq (& self . re , & other . re , epsilon . clone ()) && T :: abs_diff_eq (& self . im , & other . im , epsilon) } }
};
}
