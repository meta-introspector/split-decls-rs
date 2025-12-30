// Generated macro for impl_37 (impl)
macro_rules! Depcrate_relative_eqimpl_37 {
() => {
// Module: crate::relative_eq
// Provides: {"impl_37"}
// Dependencies: {}
impl < T : RelativeEq + Copy > RelativeEq for cell :: Cell < T > { # [inline] fn default_max_relative () -> T :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & cell :: Cell < T > , epsilon : T :: Epsilon , max_relative : T :: Epsilon ,) -> bool { T :: relative_eq (& self . get () , & other . get () , epsilon , max_relative) } }
};
}
