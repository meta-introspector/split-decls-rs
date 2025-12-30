// Generated macro for impl_38 (impl)
macro_rules! Depcrate_relative_eqimpl_38 {
() => {
// Module: crate::relative_eq
// Provides: {"impl_38"}
// Dependencies: {}
impl < T : RelativeEq + ? Sized > RelativeEq for cell :: RefCell < T > { # [inline] fn default_max_relative () -> T :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & cell :: RefCell < T > , epsilon : T :: Epsilon , max_relative : T :: Epsilon ,) -> bool { T :: relative_eq (& self . borrow () , & other . borrow () , epsilon , max_relative) } }
};
}
