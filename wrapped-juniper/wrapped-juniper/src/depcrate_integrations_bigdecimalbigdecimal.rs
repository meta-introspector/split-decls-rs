// Generated macro for BigDecimal (type)
macro_rules! Depcrate_integrations_bigdecimalBigDecimal {
() => {
// Module: crate::integrations::bigdecimal
// Provides: {"BigDecimal"}
// Dependencies: {}
# [doc = " Big decimal type."] # [doc = ""] # [doc = " Allows storing any real number to arbitrary precision; which avoids common"] # [doc = " floating point errors (such as 0.1 + 0.2 ≠ 0.3) at the cost of complexity."] # [doc = ""] # [doc = " Always serializes as `String`. But may be deserialized from `Int` and"] # [doc = " `Float` values too. It's not recommended to deserialize from a `Float`"] # [doc = " directly, as the floating point representation may be unexpected."] # [doc = ""] # [doc = " See also [`bigdecimal`] crate for details."] # [doc = ""] # [doc = " [`bigdecimal`]: https://docs.rs/bigdecimal"] # [graphql_scalar] # [graphql (with = bigdecimal_scalar , to_output_with = ScalarValue :: from_displayable , parse_token (i32 , f64 , String) , specified_by_url = "https://docs.rs/bigdecimal" ,)] type BigDecimal = bigdecimal :: BigDecimal ;
};
}
