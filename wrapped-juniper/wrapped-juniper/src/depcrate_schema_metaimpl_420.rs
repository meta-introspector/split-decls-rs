// Generated macro for impl_420 (impl)
macro_rules! Depcrate_schema_metaimpl_420 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_420"}
// Dependencies: {}
impl < S > Field < S > { # [doc = " Sets the `description` of this [`Field`]."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Adds an `argument` to this [`Field`]."] # [doc = ""] # [doc = " Arguments are unordered and can't contain duplicates by name."] # [must_use] pub fn argument (mut self , argument : Argument < S >) -> Self { match self . arguments { None => { self . arguments = Some (vec ! [argument]) ; } Some (ref mut args) => { args . push (argument) ; } } ; self } # [doc = " Indicates whether this [`Field`] is GraphQL built-in."] # [must_use] pub fn is_builtin (& self) -> bool { self . name . starts_with ("__") } # [doc = " Sets this [`Field`] as deprecated with an optional `reason`."] # [doc = ""] # [doc = " Overwrites any previously set deprecation reason."] # [must_use] pub fn deprecated (mut self , reason : Option < impl Into < ArcStr > >) -> Self { self . deprecation_status = DeprecationStatus :: Deprecated (reason . map (Into :: into)) ; self } }
};
}
