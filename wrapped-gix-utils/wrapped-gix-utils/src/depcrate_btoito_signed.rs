// Generated macro for to_signed (function)
macro_rules! Depcrate_btoito_signed {
() => {
// Module: crate::btoi
// Provides: {"to_signed"}
// Dependencies: {}
# [doc = " Converts a byte slice to an integer."] # [doc = ""] # [doc = " Like [`to_unsigned`], but numbers may optionally start with a sign (`-` or `+`)."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`ParseIntegerError`] for any of the following conditions:"] # [doc = ""] # [doc = " * `bytes` has no digits"] # [doc = " * not all characters of `bytes` are `0-9`, excluding an optional leading"] # [doc = "   sign"] # [doc = " * the number overflows or underflows `I`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics in the pathological case that there is no representation of `10`"] # [doc = " in `I`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use gix_utils::btoi::to_signed;"] # [doc = " assert_eq!(Ok(123), to_signed(b\"123\"));"] # [doc = " assert_eq!(Ok(123), to_signed(b\"+123\"));"] # [doc = " assert_eq!(Ok(-123), to_signed(b\"-123\"));"] # [doc = ""] # [doc = " assert!(to_signed::<u8>(b\"123456789\").is_err()); // overflow"] # [doc = " assert!(to_signed::<u8>(b\"-1\").is_err()); // underflow"] # [doc = ""] # [doc = " assert!(to_signed::<i32>(b\" 42\").is_err()); // leading space"] # [doc = " ```"] pub fn to_signed < I : MinNumTraits > (bytes : & [u8]) -> Result < I , ParseIntegerError > { to_signed_with_radix (bytes , 10) }
};
}
