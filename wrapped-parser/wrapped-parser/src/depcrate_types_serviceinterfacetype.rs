// Generated macro for InterfaceType (struct)
macro_rules! Depcrate_types_serviceInterfaceType {
() => {
// Module: crate::types::service
// Provides: {"InterfaceType"}
// Dependencies: {}
# [doc = " The definition of an interface type."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#InterfaceType)."] # [derive (Debug , Clone)] pub struct InterfaceType { # [doc = " The interfaces implemented by the interface."] pub implements : Vec < Positioned < Name > > , # [doc = " The fields of the interface type."] pub fields : Vec < Positioned < FieldDefinition > > , }
};
}
