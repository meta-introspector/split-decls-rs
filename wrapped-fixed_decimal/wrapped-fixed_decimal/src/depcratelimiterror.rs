// Generated macro for LimitError (struct)
macro_rules! DepcrateLimitError {
() => {
// Module: crate
// Provides: {"LimitError"}
// Dependencies: {}
# [doc = " The magnitude or number of digits exceeds the limit of the [`UnsignedDecimal`] or [`Decimal`]."] # [doc = ""] # [doc = " The highest"] # [doc = " magnitude of the most significant digit is [`i16::MAX`], and the lowest magnitude of the"] # [doc = " least significant digit is [`i16::MIN`]."] # [doc = ""] # [doc = " This error is also returned when constructing a [`FixedInteger`] from a [`Decimal`] with a"] # [doc = " fractional part."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use fixed_decimal::Decimal;"] # [doc = " use fixed_decimal::LimitError;"] # [doc = ""] # [doc = " let mut dec1 = Decimal::from(123);"] # [doc = " dec1.multiply_pow10(i16::MAX);"] # [doc = " assert!(dec1.is_zero());"] # [doc = " ```"] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [allow (clippy :: exhaustive_structs)] # [displaydoc ("Magnitude or number of digits exceeded")] pub struct LimitError ;
};
}
