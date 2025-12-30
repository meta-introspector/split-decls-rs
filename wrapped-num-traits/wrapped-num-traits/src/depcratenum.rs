// Generated macro for Num (trait)
macro_rules! DepcrateNum {
() => {
// Module: crate
// Provides: {"Num"}
// Dependencies: {}
# [doc = " The base trait for numeric types, covering `0` and `1` values,"] # [doc = " comparisons, basic numeric operations, and string conversion."] pub trait Num : PartialEq + Zero + One + NumOps { type FromStrRadixErr ; # [doc = " Convert from a string and radix (typically `2..=36`)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use num_traits::Num;"] # [doc = ""] # [doc = " let result = <i32 as Num>::from_str_radix(\"27\", 10);"] # [doc = " assert_eq!(result, Ok(27));"] # [doc = ""] # [doc = " let result = <i32 as Num>::from_str_radix(\"foo\", 10);"] # [doc = " assert!(result.is_err());"] # [doc = " ```"] # [doc = ""] # [doc = " # Supported radices"] # [doc = ""] # [doc = " The exact range of supported radices is at the discretion of each type implementation. For"] # [doc = " primitive integers, this is implemented by the inherent `from_str_radix` methods in the"] # [doc = " standard library, which **panic** if the radix is not in the range from 2 to 36. The"] # [doc = " implementation in this crate for primitive floats is similar."] # [doc = ""] # [doc = " For third-party types, it is suggested that implementations should follow suit and at least"] # [doc = " accept `2..=36` without panicking, but an `Err` may be returned for any unsupported radix."] # [doc = " It's possible that a type might not even support the common radix 10, nor any, if string"] # [doc = " parsing doesn't make sense for that type."] fn from_str_radix (str : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > ; }
};
}
