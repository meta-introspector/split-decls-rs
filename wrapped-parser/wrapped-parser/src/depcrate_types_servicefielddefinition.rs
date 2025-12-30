// Generated macro for FieldDefinition (struct)
macro_rules! Depcrate_types_serviceFieldDefinition {
() => {
// Module: crate::types::service
// Provides: {"FieldDefinition"}
// Dependencies: {}
# [doc = " The definition of a field inside an object or interface."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#FieldDefinition)."] # [derive (Debug , Clone)] pub struct FieldDefinition { # [doc = " The description of the field."] pub description : Option < Positioned < String > > , # [doc = " The name of the field."] pub name : Positioned < Name > , # [doc = " The arguments of the field."] pub arguments : Vec < Positioned < InputValueDefinition > > , # [doc = " The type of the field."] pub ty : Positioned < Type > , # [doc = " The directives of the field."] pub directives : Vec < Positioned < ConstDirective > > , }
};
}
