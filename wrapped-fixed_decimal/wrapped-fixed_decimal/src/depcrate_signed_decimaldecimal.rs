// Generated macro for Decimal (type)
macro_rules! Depcrate_signed_decimalDecimal {
() => {
// Module: crate::signed_decimal
// Provides: {"Decimal"}
// Dependencies: {}
# [doc = " A Type containing a [`UnsignedDecimal`] and a [`Sign`] to represent a signed decimal number."] # [doc = ""] # [doc = " **The primary definition of this type is in the [`fixed_decimal`](https://docs.rs/fixed_decimal) crate. Other ICU4X crates re-export it for convenience.**"] # [doc = ""] # [doc = " Supports a mantissa of non-zero digits and a number of leading and trailing"] # [doc = " zeros, as well as an optional sign; used for formatting and plural selection."] # [doc = ""] # [doc = " # Data Types"] # [doc = ""] # [doc = " The following types can be converted to a `Decimal`:"] # [doc = ""] # [doc = " - Integers, signed and unsigned"] # [doc = " - Strings representing an arbitrary-precision decimal"] # [doc = " - Floating point values (using the `ryu` feature)"] # [doc = ""] # [doc = " To create a [`Decimal`] with fractional digits, you have several options:"] # [doc = " - Create it from an integer and then call [`UnsignedDecimal::multiply_pow10`] (you can also call `multiply_pow10` directly on the [`Decimal`])."] # [doc = " - Create it from a string."] # [doc = " - When the `ryu` feature is enabled, create it from a floating point value using [`Decimal::try_from_f64`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use fixed_decimal::Decimal;"] # [doc = ""] # [doc = " let mut dec = Decimal::from(250);"] # [doc = " assert_eq!(\"250\", dec.to_string());"] # [doc = ""] # [doc = " dec.multiply_pow10(-2);"] # [doc = " assert_eq!(\"2.50\", dec.to_string());"] # [doc = " ```"] pub type Decimal = Signed < UnsignedDecimal > ;
};
}
