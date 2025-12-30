// Generated macro for impl_422 (impl)
macro_rules! Depcrate_schema_metaimpl_422 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_422"}
// Dependencies: {}
impl < S > Argument < S > { # [doc = " Builds a new [`Argument`] of the given [`Type`] with the given `name`."] pub fn new (name : impl Into < ArcStr > , arg_type : Type) -> Self { Self { name : name . into () , description : None , arg_type , default_value : None , deprecation_status : DeprecationStatus :: Current , } } # [doc = " Sets the `description` of this [`Argument`]."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Indicates whether this [`Argument`] is GraphQL built-in."] # [must_use] pub fn is_builtin (& self) -> bool { self . name . starts_with ("__") } # [doc = " Sets the default value of this [`Argument`]."] # [doc = ""] # [doc = " Overwrites any previously set default value."] # [must_use] pub fn default_value (mut self , val : InputValue < S >) -> Self { self . default_value = Some (val) ; self } # [doc = " Sets this [`Argument`] as deprecated with an optional `reason`."] # [doc = ""] # [doc = " Overwrites any previously set deprecation reason."] # [must_use] pub fn deprecated (mut self , reason : Option < impl Into < ArcStr > >) -> Self { self . deprecation_status = DeprecationStatus :: Deprecated (reason . map (Into :: into)) ; self } }
};
}
