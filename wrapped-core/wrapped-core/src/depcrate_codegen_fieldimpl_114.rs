// Generated macro for impl_114 (impl)
macro_rules! Depcrate_codegen_fieldimpl_114 {
() => {
// Module: crate::codegen::field
// Provides: {"impl_114"}
// Dependencies: {}
impl < 'a > Field < 'a > { # [doc = " Get the name of the meta item that should be matched against input and should be used in diagnostics."] # [doc = ""] # [doc = " This will be `None` if the field is `skip` or `flatten`, as neither kind of field is addressable"] # [doc = " by name from the input meta."] pub fn as_name (& 'a self) -> Option < & 'a str > { if self . skip || self . flatten { None } else { Some (& self . name_in_attr) } } pub fn as_declaration (& 'a self) -> Declaration < 'a > { Declaration (self) } pub fn as_flatten_initializer (& 'a self , parent_field_names : Vec < & 'a str > ,) -> FlattenInitializer < 'a > { FlattenInitializer { field : self , parent_field_names , } } pub fn as_match (& 'a self) -> MatchArm < 'a > { MatchArm (self) } pub fn as_initializer (& 'a self) -> Initializer < 'a > { Initializer (self) } pub fn as_presence_check (& 'a self) -> CheckMissing < 'a > { CheckMissing (self) } }
};
}
