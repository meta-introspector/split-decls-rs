// Generated macro for EnumValueDefinition (struct)
macro_rules! Depcrate_types_serviceEnumValueDefinition {
() => {
// Module: crate::types::service
// Provides: {"EnumValueDefinition"}
// Dependencies: {}
# [doc = " The definition of a value inside an enum."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#EnumValueDefinition)."] # [derive (Debug , Clone)] pub struct EnumValueDefinition { # [doc = " The description of the argument."] pub description : Option < Positioned < String > > , # [doc = " The value name."] pub value : Positioned < Name > , # [doc = " The directives of the enum value."] pub directives : Vec < Positioned < ConstDirective > > , }
};
}
