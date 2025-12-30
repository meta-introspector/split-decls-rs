// Generated macro for impl_40 (impl)
macro_rules! Depcrate_relative_eqimpl_40 {
() => {
// Module: crate::relative_eq
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg (feature = "num-complex")] impl < T : RelativeEq > RelativeEq for Complex < T > where T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> T :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Complex < T > , epsilon : T :: Epsilon , max_relative : T :: Epsilon ,) -> bool { T :: relative_eq (& self . re , & other . re , epsilon . clone () , max_relative . clone ()) && T :: relative_eq (& self . im , & other . im , epsilon , max_relative) } }
};
}
