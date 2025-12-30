// Generated macro for impl_7 (impl)
macro_rules! Depcrate_compactimpl_7 {
() => {
// Module: crate::compact
// Provides: {"impl_7"}
// Dependencies: {}
impl CompactDecimal { # [doc = " Constructs a [`CompactDecimal`] from its significand and exponent."] pub fn from_significand_and_exponent (significand : Decimal , exponent : u8) -> Self { Self { significand , exponent , } } # [doc = " Returns a reference to the significand of `self`."] # [doc = " ```"] # [doc = " # use fixed_decimal::CompactDecimal;"] # [doc = " # use fixed_decimal::Decimal;"] # [doc = " # use std::str::FromStr;"] # [doc = " #"] # [doc = " assert_eq!("] # [doc = "     CompactDecimal::from_str(\"+1.20c6\").unwrap().significand(),"] # [doc = "     &Decimal::from_str(\"+1.20\").unwrap()"] # [doc = " );"] # [doc = " ```"] pub fn significand (& self) -> & Decimal { & self . significand } # [doc = " Returns the significand of `self`."] # [doc = " ```"] # [doc = " # use fixed_decimal::CompactDecimal;"] # [doc = " # use fixed_decimal::Decimal;"] # [doc = " # use std::str::FromStr;"] # [doc = " #"] # [doc = " assert_eq!("] # [doc = "     CompactDecimal::from_str(\"+1.20c6\")"] # [doc = "         .unwrap()"] # [doc = "         .into_significand(),"] # [doc = "     Decimal::from_str(\"+1.20\").unwrap()"] # [doc = " );"] # [doc = " ```"] pub fn into_significand (self) -> Decimal { self . significand } # [doc = " Returns the exponent of `self`."] # [doc = " ```"] # [doc = " # use fixed_decimal::CompactDecimal;"] # [doc = " # use std::str::FromStr;"] # [doc = " #"] # [doc = " assert_eq!(CompactDecimal::from_str(\"+1.20c6\").unwrap().exponent(), 6);"] # [doc = " assert_eq!(CompactDecimal::from_str(\"1729\").unwrap().exponent(), 0);"] # [doc = " ```"] pub fn exponent (& self) -> u8 { self . exponent } }
};
}
