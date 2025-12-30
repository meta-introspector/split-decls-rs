// Generated macro for impl_35 (impl)
macro_rules! Depcrate_relative_eqimpl_35 {
() => {
// Module: crate::relative_eq
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , T : RelativeEq + ? Sized > RelativeEq for & 'a T { # [inline] fn default_max_relative () -> T :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & & 'a T , epsilon : T :: Epsilon , max_relative : T :: Epsilon) -> bool { T :: relative_eq (* self , * other , epsilon , max_relative) } }
};
}
