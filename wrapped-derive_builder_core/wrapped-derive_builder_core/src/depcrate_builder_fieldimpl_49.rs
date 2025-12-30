// Generated macro for impl_49 (impl)
macro_rules! Depcrate_builder_fieldimpl_49 {
() => {
// Module: crate::builder_field
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > BuilderFieldType < 'a > { # [doc = " Obtain type information for the builder field setter"] # [doc = ""] # [doc = " Return value:"] # [doc = "  * `.0`: type of the argument to the setter function (before application of `strip_option`, `into`)"] # [doc = "  * `.1`: whether the builder field is `Option<type>` rather than just `type`"] pub fn setter_type_info (& 'a self) -> (& 'a syn :: Type , bool) { match self { BuilderFieldType :: Optional (ty) => (ty , true) , BuilderFieldType :: Precise (ty) => (ty , false) , BuilderFieldType :: Phantom (_ty) => panic ! ("phantom fields should never have setters") , } } fn with_crate_root (& 'a self , crate_root : & 'a syn :: Path) -> BuilderFieldTypeWithCrateRoot < 'a > { BuilderFieldTypeWithCrateRoot { crate_root , field_type : self , } } }
};
}
