// Generated macro for Decimal (type)
macro_rules! Depcrate_integrations_rust_decimalDecimal {
() => {
// Module: crate::integrations::rust_decimal
// Provides: {"Decimal"}
// Dependencies: {}
# [doc = " 128 bit representation of a fixed-precision decimal number."] # [doc = ""] # [doc = " The finite set of values of `Decimal` scalar are of the form"] # [doc = " m / 10<sup>e</sup>, where m is an integer such that"] # [doc = " -2<sup>96</sup> < m < 2<sup>96</sup>, and e is an integer between 0 and 28"] # [doc = " inclusive."] # [doc = ""] # [doc = " Always serializes as `String`. But may be deserialized from `Int` and"] # [doc = " `Float` values too. It's not recommended to deserialize from a `Float`"] # [doc = " directly, as the floating point representation may be unexpected."] # [doc = ""] # [doc = " See also [`rust_decimal`] crate for details."] # [doc = ""] # [doc = " [`rust_decimal`]: https://docs.rs/rust_decimal"] # [graphql_scalar] # [graphql (with = rust_decimal_scalar , to_output_with = ScalarValue :: from_displayable , parse_token (i32 , f64 , String) , specified_by_url = "https://docs.rs/rust_decimal" ,)] type Decimal = rust_decimal :: Decimal ;
};
}
