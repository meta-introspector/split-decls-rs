// Generated macro for impl_36 (impl)
macro_rules! Depcrate_relative_eqimpl_36 {
() => {
// Module: crate::relative_eq
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a , T : RelativeEq + ? Sized > RelativeEq for & 'a mut T { # [inline] fn default_max_relative () -> T :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & & 'a mut T , epsilon : T :: Epsilon , max_relative : T :: Epsilon ,) -> bool { T :: relative_eq (* self , * other , epsilon , max_relative) } }
};
}
