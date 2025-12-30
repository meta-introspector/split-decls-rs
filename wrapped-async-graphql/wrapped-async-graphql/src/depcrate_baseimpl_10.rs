// Generated macro for impl_10 (impl)
macro_rules! Depcrate_baseimpl_10 {
() => {
// Module: crate::base
// Provides: {"impl_10"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + ? Sized > OutputType for & T { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } # [allow (clippy :: trivially_copy_pass_by_ref)] async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { T :: resolve (* self , ctx , field) . await } }
};
}
