// Generated macro for impl_143 (impl)
macro_rules! Depcrate_astimpl_143 {
() => {
// Module: crate::ast
// Provides: {"impl_143"}
// Dependencies: {}
impl < N , M : Extend < TypeModifier > > Type < N , M > { # [doc = " Wraps this [`Type`] into the provided [`TypeModifier`]."] fn wrap (mut self , modifier : TypeModifier) -> Self { self . modifiers . extend ([modifier]) ; self } # [doc = " Wraps this [`Type`] into a [`List`] with the provided `expected_size`, if any."] # [must_use] pub fn wrap_list (self , expected_size : Option < usize >) -> Self { self . wrap (TypeModifier :: List (expected_size)) } # [doc = " Wraps this [`Type`] as a [`NonNull`] one."] # [must_use] pub fn wrap_non_null (self) -> Self { self . wrap (TypeModifier :: NonNull) } }
};
}
