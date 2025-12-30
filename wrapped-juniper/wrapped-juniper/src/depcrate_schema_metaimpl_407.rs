// Generated macro for impl_407 (impl)
macro_rules! Depcrate_schema_metaimpl_407 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_407"}
// Dependencies: {}
impl NullableMeta { # [doc = " Builds a new [`NullableMeta`] type by wrapping the specified [`Type`]."] pub fn new (of_type : Type) -> Self { Self { of_type } } # [doc = " Wraps this [`NullableMeta`] type into a generic [`MetaType`]."] pub fn into_meta < S > (self) -> MetaType < S > { MetaType :: Nullable (self) } }
};
}
