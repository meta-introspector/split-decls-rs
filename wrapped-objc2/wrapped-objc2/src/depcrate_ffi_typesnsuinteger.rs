// Generated macro for NSUInteger (type)
macro_rules! Depcrate_ffi_typesNSUInteger {
() => {
// Module: crate::ffi::types
// Provides: {"NSUInteger"}
// Dependencies: {}
# [doc = " Describes an unsigned integer."] # [doc = ""] # [doc = " This is guaranteed to always be a type-alias to [`usize`]. That means it"] # [doc = " is valid to use `#[repr(usize)]` on enums and structs with size"] # [doc = " `NSUInteger`."] # [doc = ""] # [doc = " See also the [corresponding documentation entry][docs]."] # [doc = ""] # [doc = " [docs]: https://developer.apple.com/documentation/objectivec/nsuinteger?language=objc"] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2::ffi::NSUInteger;"] # [doc = ""] # [doc = " extern \"C-unwind\" {"] # [doc = "     fn some_external_function() -> NSUInteger;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use core::mem::size_of;"] # [doc = " use objc2::ffi::NSUInteger;"] # [doc = ""] # [doc = " #[repr(usize)]"] # [doc = " enum CLRegionState {"] # [doc = "     Unknown = 0,"] # [doc = "     Inside = 1,"] # [doc = "     Outside = 2,"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(size_of::<CLRegionState>(), size_of::<NSUInteger>());"] # [doc = " ```"] pub type NSUInteger = usize ;
};
}
