// Generated macro for ValueDefinition (struct)
macro_rules! Depcrate_graphql_enumValueDefinition {
() => {
// Module: crate::graphql_enum
// Provides: {"ValueDefinition"}
// Dependencies: {}
# [doc = " Representation of a [GraphQL enum value][1] for code generation."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Enum-Value"] # [derive (Debug)] struct ValueDefinition { # [doc = " [`Ident`] of the Rust enum variant behind this [GraphQL enum value][1]."] # [doc = ""] # [doc = " [`Ident`]: syn::Ident"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Enum-Value"] ident : syn :: Ident , # [doc = " Name of this [GraphQL enum value][1] in GraphQL schema."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Enum-Value"] name : Box < str > , # [doc = " [Description][2] of this [GraphQL enum value][1] to put into GraphQL"] # [doc = " schema."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Enum-Value"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Descriptions"] description : Option < Description > , # [doc = " [Deprecation][2] of this [GraphQL enum value][1] to put into GraphQL"] # [doc = " schema."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Enum-Value"] # [doc = " [2]: https://spec.graphql.org/October2021#sec--deprecated"] deprecated : Option < deprecation :: Directive > , }
};
}
