// Generated macro for ObjectType (struct)
macro_rules! Depcrate_types_serviceObjectType {
() => {
// Module: crate::types::service
// Provides: {"ObjectType"}
// Dependencies: {}
# [doc = " The definition of an object type."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#ObjectType)."] # [derive (Debug , Clone)] pub struct ObjectType { # [doc = " The interfaces implemented by the object."] pub implements : Vec < Positioned < Name > > , # [doc = " The fields of the object type."] pub fields : Vec < Positioned < FieldDefinition > > , }
};
}
