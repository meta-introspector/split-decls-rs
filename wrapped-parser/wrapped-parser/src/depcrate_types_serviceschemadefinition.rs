// Generated macro for SchemaDefinition (struct)
macro_rules! Depcrate_types_serviceSchemaDefinition {
() => {
// Module: crate::types::service
// Provides: {"SchemaDefinition"}
// Dependencies: {}
# [doc = " The definition of the schema in a GraphQL service."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#SchemaDefinition). This also covers"] # [doc = " [extensions](https://spec.graphql.org/October2021/#SchemaExtension)."] # [derive (Debug , Clone)] pub struct SchemaDefinition { # [doc = " Whether the schema is an extension of another schema."] pub extend : bool , # [doc = " The directives of the schema definition."] pub directives : Vec < Positioned < ConstDirective > > , # [doc = " The query root. This is always `Some` when the schema is not extended."] pub query : Option < Positioned < Name > > , # [doc = " The mutation root, if present."] pub mutation : Option < Positioned < Name > > , # [doc = " The subscription root, if present."] pub subscription : Option < Positioned < Name > > , }
};
}
