// Generated macro for to_unsigned (function)
macro_rules! Depcrate_btoito_unsigned {
() => {
// Module: crate::btoi
// Provides: {"to_unsigned"}
// Dependencies: {}
# [doc = " Converts a byte slice to an integer. Signs are not allowed."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`ParseIntegerError`] for any of the following conditions:"] # [doc = ""] # [doc = " * `bytes` is empty"] # [doc = " * not all characters of `bytes` are `0-9`"] # [doc = " * the number overflows `I`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics in the pathological case that there is no representation of `10`"] # [doc = " in `I`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use gix_utils::btoi::to_unsigned;"] # [doc = " assert_eq!(Ok(12345), to_unsigned(b\"12345\"));"] # [doc = " assert!(to_unsigned::<u8>(b\"+1\").is_err()); // only btoi allows signs"] # [doc = " assert!(to_unsigned::<u8>(b\"256\").is_err()); // overflow"] # [doc = " ```"] # [track_caller] pub fn to_unsigned < I : MinNumTraits > (bytes : & [u8]) -> Result < I , ParseIntegerError > { to_unsigned_with_radix (bytes , 10) }
};
}
