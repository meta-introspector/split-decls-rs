// Generated macro for GraphQLInterface (trait)
macro_rules! Depcrate_types_markerGraphQLInterface {
() => {
// Module: crate::types::marker
// Provides: {"GraphQLInterface"}
// Dependencies: {}
# [doc = " Maker trait for [GraphQL interfaces][1]."] # [doc = ""] # [doc = " This trait extends the [`GraphQLType`] and is only used to mark an [interface][1]. During"] # [doc = " compile this addition information is required to prevent unwanted structure compiling. If an"] # [doc = " object requires this trait instead of the [`GraphQLType`], then it explicitly requires"] # [doc = " [GraphQL interfaces][1]. Other types ([scalars][2], [enums][3], [objects][4], [input objects][5]"] # [doc = " and [unions][6]) are not allowed."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Interfaces"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Scalars"] # [doc = " [3]: https://spec.graphql.org/October2021#sec-Enums"] # [doc = " [4]: https://spec.graphql.org/October2021#sec-Objects"] # [doc = " [5]: https://spec.graphql.org/October2021#sec-Input-Objects"] # [doc = " [6]: https://spec.graphql.org/October2021#sec-Unions"] pub trait GraphQLInterface < S : ScalarValue > : GraphQLType < S > { # [doc = " An arbitrary function without meaning."] # [doc = ""] # [doc = " May contain compile timed check logic which ensures that types are used correctly according"] # [doc = " to the [GraphQL specification][1]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021"] fn mark () { } }
};
}
