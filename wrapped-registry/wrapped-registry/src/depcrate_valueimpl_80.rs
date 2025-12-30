// Generated macro for impl_80 (impl)
macro_rules! Depcrate_valueimpl_80 {
() => {
// Module: crate::value
// Provides: {"impl_80"}
// Dependencies: {}
impl Value { # [doc = " Gets the type of the registry value."] pub fn ty (& self) -> Type { self . ty } # [doc = " Sets the type of the registry value. This does not change the value."] pub fn set_ty (& mut self , ty : Type) { self . ty = ty ; } # [doc = " Gets the value as a slice of u16 for raw wide characters."] pub fn as_wide (& self) -> & [u16] { self . data . as_wide () } }
};
}
