// Generated macro for impl_411 (impl)
macro_rules! Depcrate_schema_metaimpl_411 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_411"}
// Dependencies: {}
impl < S > EnumMeta < S > { # [doc = " Builds a new [`EnumMeta`] type with the specified `name` and possible `values`."] pub fn new < T > (name : impl Into < ArcStr > , values : & [EnumValue]) -> Self where T : FromInputValue < S > , T :: Error : IntoFieldError < S > , { Self { name : name . into () , description : None , values : values . to_owned () , try_parse_fn : try_parse_fn :: < S , T > , } } # [doc = " Sets the `description` of this [`EnumMeta`] type."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Wraps this [`EnumMeta`] type into a generic [`MetaType`]."] pub fn into_meta (self) -> MetaType < S > { MetaType :: Enum (self) } }
};
}
