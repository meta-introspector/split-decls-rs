// Generated macro for impl_34 (impl)
macro_rules! Depcrate_decimalimpl_34 {
() => {
// Module: crate::decimal
// Provides: {"impl_34"}
// Dependencies: {}
# [doc = " Render the `FixedDecimal` as a string of ASCII digits with a possible decimal point."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use fixed_decimal::UnsignedDecimal;"] # [doc = " # use writeable::assert_writeable_eq;"] # [doc = " #"] # [doc = " assert_writeable_eq!(UnsignedDecimal::from(42u32), \"42\");"] # [doc = " ```"] impl writeable :: Writeable for UnsignedDecimal { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { for m in self . magnitude_range () . rev () { if m == - 1 { sink . write_char ('.') ? ; } let d = self . digit_at (m) ; sink . write_char ((b'0' + d) as char) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { writeable :: LengthHint :: exact (1) + ((self . upper_magnitude as i32 - self . lower_magnitude as i32) as usize) + (self . lower_magnitude < 0) as usize } }
};
}
