// Generated macro for impl_2018 (impl)
macro_rules! Depcrate_geometry_translationimpl_2018 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2018"}
// Dependencies: {}
impl < T : Scalar + AbsDiffEq , const D : usize > AbsDiffEq for Translation < T , D > where T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . vector . abs_diff_eq (& other . vector , epsilon) } }
};
}
