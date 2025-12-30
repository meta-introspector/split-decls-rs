// Generated macro for Directive (struct)
macro_rules! Depcrate_common_deprecationDirective {
() => {
// Module: crate::common::deprecation
// Provides: {"Directive"}
// Dependencies: {}
# [doc = " [GraphQL deprecation directive][0] defined on a [GraphQL field][1] or a"] # [doc = " [GraphQL enum value][2] via `#[graphql(deprecated = ...)]` (or"] # [doc = " `#[deprecated(note = ...)]`) attribute."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec--deprecated"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Language.Fields"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Enum-Value"] # [derive (Debug , Default , Eq , PartialEq)] pub (crate) struct Directive { # [doc = " Optional [reason][1] attached to this [deprecation][0]."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec--deprecated"] # [doc = " [1]: https://spec.graphql.org/October2021#sel-GAHnBZDACEDDGAA_6L"] pub (crate) reason : Option < syn :: LitStr > , }
};
}
