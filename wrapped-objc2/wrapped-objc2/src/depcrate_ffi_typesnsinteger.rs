// Generated macro for NSInteger (type)
macro_rules! Depcrate_ffi_typesNSInteger {
() => {
// Module: crate::ffi::types
// Provides: {"NSInteger"}
// Dependencies: {}
# [doc = " A signed integer value type."] # [doc = ""] # [doc = " This is guaranteed to always be a type-alias to [`isize`]. That means it"] # [doc = " is valid to use `#[repr(isize)]` on enums and structs with size"] # [doc = " `NSInteger`."] # [doc = ""] # [doc = " See also the [corresponding documentation entry][docs]."] # [doc = ""] # [doc = " [docs]: https://developer.apple.com/documentation/objectivec/nsinteger?language=objc"] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::mem::size_of;"] # [doc = " use objc2::ffi::NSInteger;"] # [doc = ""] # [doc = " #[repr(isize)]"] # [doc = " pub enum NSComparisonResult {"] # [doc = "     NSOrderedAscending = -1,"] # [doc = "     NSOrderedSame = 0,"] # [doc = "     NSOrderedDescending = 1,"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(size_of::<NSComparisonResult>(), size_of::<NSInteger>());"] # [doc = " ```"] pub type NSInteger = isize ;
};
}
