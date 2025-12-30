// Generated macro for DynClone (trait)
macro_rules! DepcrateDynClone {
() => {
// Module: crate
// Provides: {"DynClone"}
// Dependencies: {}
# [doc = " This trait is implemented by any type that implements [`std::clone::Clone`]."] pub trait DynClone : Sealed { # [doc (hidden)] fn __clone_box (& self , _ : Private) -> * mut () ; }
};
}
