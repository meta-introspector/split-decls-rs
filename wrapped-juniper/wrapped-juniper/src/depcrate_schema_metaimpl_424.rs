// Generated macro for impl_424 (impl)
macro_rules! Depcrate_schema_metaimpl_424 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_424"}
// Dependencies: {}
impl EnumValue { # [doc = " Constructs a new [`EnumValue`] with the provided `name`."] pub fn new (name : impl Into < ArcStr >) -> Self { Self { name : name . into () , description : None , deprecation_status : DeprecationStatus :: Current , } } # [doc = " Sets the `description` of this [`EnumValue`]."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Sets this [`EnumValue`] as deprecated with an optional `reason`."] # [doc = ""] # [doc = " Overwrites any previously set deprecation reason."] # [must_use] pub fn deprecated (mut self , reason : Option < impl Into < ArcStr > >) -> Self { self . deprecation_status = DeprecationStatus :: Deprecated (reason . map (Into :: into)) ; self } }
};
}
