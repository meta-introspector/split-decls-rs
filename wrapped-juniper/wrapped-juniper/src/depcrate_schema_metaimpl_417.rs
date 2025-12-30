// Generated macro for impl_417 (impl)
macro_rules! Depcrate_schema_metaimpl_417 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_417"}
// Dependencies: {}
impl < S > InputObjectMeta < S > { # [doc = " Builds a new [`InputObjectMeta`] type with the specified `name` and `input_fields`."] pub fn new < T > (name : impl Into < ArcStr > , input_fields : & [Argument < S >]) -> Self where T : FromInputValue < S > , T :: Error : IntoFieldError < S > , S : Clone , { Self { name : name . into () , description : None , input_fields : input_fields . to_vec () , is_one_of : false , try_parse_fn : try_parse_fn :: < S , T > , } } # [doc = " Sets the `description` of this [`InputObjectMeta`] type."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Marks this [`InputObjectMeta`] type as [`@oneOf`]."] # [doc = ""] # [doc = " [`@oneOf`]: https://spec.graphql.org/September2025#sec--oneOf"] # [must_use] pub fn one_of (mut self) -> Self { self . is_one_of = true ; self } # [doc = " Wraps this [`InputObjectMeta`] type into a generic [`MetaType`]."] pub fn into_meta (self) -> MetaType < S > { MetaType :: InputObject (self) } }
};
}
