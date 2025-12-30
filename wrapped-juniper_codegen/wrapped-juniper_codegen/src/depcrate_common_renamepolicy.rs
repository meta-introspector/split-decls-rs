// Generated macro for Policy (enum)
macro_rules! Depcrate_common_renamePolicy {
() => {
// Module: crate::common::rename
// Provides: {"Policy"}
// Dependencies: {}
# [doc = " Possible ways to rename all [GraphQL fields][1] or [GrqphQL enum values][2]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Language.Fields"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Enum-Value"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub (crate) enum Policy { # [doc = " Do nothing, and use the default conventions renaming."] None , # [doc = " Rename in `camelCase` style."] CamelCase , # [doc = " Rename in `snake_case` style."] SnakeCase , # [doc = " Rename in `SCREAMING_SNAKE_CASE` style."] ScreamingSnakeCase , }
};
}
