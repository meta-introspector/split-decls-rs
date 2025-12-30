// Generated macro for impl_104 (impl)
macro_rules! Depcrate_scientificimpl_104 {
() => {
// Module: crate::scientific
// Provides: {"impl_104"}
// Dependencies: {}
# [doc = " Render the [`ScientificDecimal`] as a string of ASCII digits with a possible decimal point,"] # [doc = " followed by the letter 'e', and the exponent."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use fixed_decimal::Decimal;"] # [doc = " # use fixed_decimal::FixedInteger;"] # [doc = " # use fixed_decimal::ScientificDecimal;"] # [doc = " # use std::str::FromStr;"] # [doc = " # use writeable::assert_writeable_eq;"] # [doc = " #"] # [doc = " assert_writeable_eq!("] # [doc = "     ScientificDecimal::from("] # [doc = "         {"] # [doc = "             let mut dec = Decimal::from(1729u32);"] # [doc = "             dec.multiply_pow10(-3);"] # [doc = "             dec"] # [doc = "         },"] # [doc = "         FixedInteger::from(3)"] # [doc = "     ),"] # [doc = "     \"1.729e3\""] # [doc = " );"] # [doc = " assert_writeable_eq!("] # [doc = "     ScientificDecimal::from("] # [doc = "         Decimal::from_str(\"+1.729\").unwrap(),"] # [doc = "         FixedInteger::from_str(\"+03\").unwrap()"] # [doc = "     ),"] # [doc = "     \"+1.729e+03\""] # [doc = " );"] # [doc = " ```"] impl writeable :: Writeable for ScientificDecimal { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { self . significand . write_to (sink) ? ; sink . write_char ('e') ? ; self . exponent . write_to (sink) } fn writeable_length_hint (& self) -> writeable :: LengthHint { self . significand . writeable_length_hint () + 1 + self . exponent . writeable_length_hint () } }
};
}
