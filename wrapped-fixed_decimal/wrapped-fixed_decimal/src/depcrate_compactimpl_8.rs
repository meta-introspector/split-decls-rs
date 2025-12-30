// Generated macro for impl_8 (impl)
macro_rules! Depcrate_compactimpl_8 {
() => {
// Module: crate::compact
// Provides: {"impl_8"}
// Dependencies: {}
# [doc = " Render the [`CompactDecimal`] in sampleValue syntax."] # [doc = " The letter c is used, rather than the deprecated e."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use fixed_decimal::CompactDecimal;"] # [doc = " # use std::str::FromStr;"] # [doc = " # use writeable::assert_writeable_eq;"] # [doc = " #"] # [doc = " assert_writeable_eq!("] # [doc = "     CompactDecimal::from_str(\"+1.20c6\").unwrap(),"] # [doc = "     \"+1.20c6\""] # [doc = " );"] # [doc = " assert_writeable_eq!(CompactDecimal::from_str(\"+1729\").unwrap(), \"+1729\");"] # [doc = " ```"] impl writeable :: Writeable for CompactDecimal { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { self . significand . write_to (sink) ? ; if self . exponent != 0 { sink . write_char ('c') ? ; self . exponent . write_to (sink) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { let mut result = self . significand . writeable_length_hint () ; if self . exponent != 0 { result += self . exponent . writeable_length_hint () + 1 ; } result } }
};
}
