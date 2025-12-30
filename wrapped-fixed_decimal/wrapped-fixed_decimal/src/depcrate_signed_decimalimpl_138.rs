// Generated macro for impl_138 (impl)
macro_rules! Depcrate_signed_decimalimpl_138 {
() => {
// Module: crate::signed_decimal
// Provides: {"impl_138"}
// Dependencies: {}
# [doc = " Render the [`Decimal`] as a string of ASCII digits with a possible decimal point."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use fixed_decimal::Decimal;"] # [doc = " # use writeable::assert_writeable_eq;"] # [doc = " #"] # [doc = " assert_writeable_eq!(Decimal::from(42), \"42\");"] # [doc = " ```"] impl writeable :: Writeable for Decimal { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { match self . sign { Sign :: Negative => sink . write_char ('-') ? , Sign :: Positive => sink . write_char ('+') ? , Sign :: None => () , } for m in self . absolute . magnitude_range () . rev () { if m == - 1 { sink . write_char ('.') ? ; } let d = self . absolute . digit_at (m) ; sink . write_char ((b'0' + d) as char) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { self . absolute . writeable_length_hint () + (self . sign != Sign :: None) as usize } }
};
}
