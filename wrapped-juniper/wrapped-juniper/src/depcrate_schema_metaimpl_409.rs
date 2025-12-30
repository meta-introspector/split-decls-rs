// Generated macro for impl_409 (impl)
macro_rules! Depcrate_schema_metaimpl_409 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_409"}
// Dependencies: {}
impl < S > ObjectMeta < S > { # [doc = " Builds a new [`ObjectMeta`] type with the specified `name` and `fields`."] pub fn new (name : impl Into < ArcStr > , fields : & [Field < S >]) -> Self where S : Clone , { Self { name : name . into () , description : None , fields : fields . to_vec () , interface_names : vec ! [] , } } # [doc = " Sets the `description` of this [`ObjectMeta`] type."] # [doc = ""] # [doc = " Overwrites any previously set description."] # [must_use] pub fn description (mut self , description : impl Into < ArcStr >) -> Self { self . description = Some (description . into ()) ; self } # [doc = " Sets the `interfaces` this [`ObjectMeta`] type implements."] # [doc = ""] # [doc = " Overwrites any previously set list of interfaces."] # [must_use] pub fn interfaces (mut self , interfaces : & [Type]) -> Self { self . interface_names = interfaces . iter () . map (| t | t . innermost_name () . into ()) . collect () ; self } # [doc = " Wraps this [`ObjectMeta`] type into a generic [`MetaType`]."] pub fn into_meta (self) -> MetaType < S > { MetaType :: Object (self) } }
};
}
