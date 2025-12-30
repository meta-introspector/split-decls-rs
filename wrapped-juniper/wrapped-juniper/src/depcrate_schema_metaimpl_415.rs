// Generated macro for impl_415 (impl)
macro_rules! Depcrate_schema_metaimpl_415 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_415"}
// Dependencies: {}
impl UnionMeta { # [doc = " Builds a new [`UnionMeta`] type with the specified `name` and possible [`Type`]s."] pub fn new (name : impl Into < ArcStr > , of_types : & [Type]) -> Self { Self { name : name . into () , description : None , of_type_names : of_types . iter () . map (| t | t . innermost_name () . into ()) . collect () , } } # [doc = " Sets the `description` of this [`UnionMeta`] type."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Wraps this [`UnionMeta`] type into a generic [`MetaType`]."] pub fn into_meta < S > (self) -> MetaType < S > { MetaType :: Union (self) } }
};
}
