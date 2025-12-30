// Generated macro for IntrospectionFormat (enum)
macro_rules! Depcrate_introspectionIntrospectionFormat {
() => {
// Module: crate::introspection
// Provides: {"IntrospectionFormat"}
// Dependencies: {}
# [doc = " Desired GraphQL introspection format for the [canonical introspection query][0]."] # [doc = ""] # [doc = " [0]: https://github.com/graphql/graphql-js/blob/v16.11.0/src/utilities/getIntrospectionQuery.ts#L75"] # [derive (Clone , Copy , Debug , Default)] pub enum IntrospectionFormat { # [doc = " The canonical GraphQL introspection query."] # [default] All , # [doc = " The canonical GraphQL introspection query without descriptions."] WithoutDescriptions , }
};
}
