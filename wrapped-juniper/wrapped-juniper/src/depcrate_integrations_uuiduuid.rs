// Generated macro for Uuid (type)
macro_rules! Depcrate_integrations_uuidUuid {
() => {
// Module: crate::integrations::uuid
// Provides: {"Uuid"}
// Dependencies: {}
# [doc = " [Universally Unique Identifier][0] (UUID)."] # [doc = ""] # [doc = " [`UUID` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`uuid::Uuid`][2] for details."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/Universally_unique_identifier"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/uuid"] # [doc = " [2]: https://docs.rs/uuid/*/uuid/struct.Uuid.html"] # [graphql_scalar] # [graphql (name = "UUID" , with = uuid_scalar , to_output_with = ScalarValue :: from_displayable , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/uuid" ,)] type Uuid = uuid :: Uuid ;
};
}
