// Generated macro for impl_405 (impl)
macro_rules! Depcrate_schema_metaimpl_405 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_405"}
// Dependencies: {}
impl ListMeta { # [doc = " Builds a new [`ListMeta`] type by wrapping the specified [`Type`]."] # [doc = ""] # [doc = " Specifying `expected_size` will be used to ensure that values of this type will always match"] # [doc = " it."] pub fn new (of_type : Type , expected_size : Option < usize >) -> Self { Self { of_type , expected_size , } } # [doc = " Wraps this [`ListMeta`] type into a generic [`MetaType`]."] pub fn into_meta < S > (self) -> MetaType < S > { MetaType :: List (self) } }
};
}
