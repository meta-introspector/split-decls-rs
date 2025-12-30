// Generated macro for impl_401 (impl)
macro_rules! Depcrate_schema_metaimpl_401 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_401"}
// Dependencies: {}
impl < S > ScalarMeta < S > { # [doc = " Builds a new [`ScalarMeta`] type with the specified `name`."] pub fn new < T > (name : impl Into < ArcStr >) -> Self where T : FromInputValue < S > + ParseScalarValue < S > , T :: Error : IntoFieldError < S > , { Self { name : name . into () , description : None , specified_by_url : None , try_parse_fn : try_parse_fn :: < S , T > , parse_fn : < T as ParseScalarValue < S > > :: from_str , } } # [doc = " Sets the `description` of this [`ScalarMeta`] type."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Sets the [specification URL][0] for this [`ScalarMeta`] type."] # [doc = ""] # [doc = " Overwrites any previously set [specification URL][0]."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec--specifiedBy"] # [must_use] pub fn specified_by_url (mut self , url : impl Into < ArcStr >) -> Self { self . specified_by_url = Some (url . into ()) ; self } # [doc = " Wraps this [`ScalarMeta`] type into a generic [`MetaType`]."] pub fn into_meta (self) -> MetaType < S > { MetaType :: Scalar (self) } }
};
}
