// Generated macro for FixedInteger (struct)
macro_rules! Depcrate_integerFixedInteger {
() => {
// Module: crate::integer
// Provides: {"FixedInteger"}
// Dependencies: {}
# [doc = " A [`FixedInteger`] is a [`Decimal`] with no fractional part."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::str::FromStr;"] # [doc = " use fixed_decimal::Decimal;"] # [doc = " use fixed_decimal::FixedInteger;"] # [doc = " use fixed_decimal::LimitError;"] # [doc = ""] # [doc = " assert_eq!(Decimal::from(FixedInteger::from(5)), Decimal::from(5));"] # [doc = " assert_eq!("] # [doc = "     FixedInteger::try_from(Decimal::from(5)),"] # [doc = "     Ok(FixedInteger::from(5))"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     FixedInteger::try_from(Decimal::from_str(\"05\").unwrap()),"] # [doc = "     Ok(FixedInteger::from_str(\"05\").unwrap())"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     FixedInteger::try_from(Decimal::from_str(\"5.0\").unwrap()),"] # [doc = "     Err(LimitError)"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq , Default)] pub struct FixedInteger (Decimal) ;
};
}
