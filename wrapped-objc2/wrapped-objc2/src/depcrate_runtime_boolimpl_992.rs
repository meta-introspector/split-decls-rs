// Generated macro for impl_992 (impl)
macro_rules! Depcrate_runtime_boolimpl_992 {
() => {
// Module: crate::runtime::bool
// Provides: {"impl_992"}
// Dependencies: {}
impl Bool { # [doc = " The equivalent of [`true`] for Objective-C's `BOOL` type."] # [allow (clippy :: unnecessary_cast)] pub const YES : Self = Self :: from_raw (true as inner :: BOOL) ; # [doc = " The equivalent of [`false`] for Objective-C's `BOOL` type."] # [allow (clippy :: unnecessary_cast)] pub const NO : Self = Self :: from_raw (false as inner :: BOOL) ; # [doc = " Creates an Objective-C boolean from a Rust boolean."] # [inline] pub const fn new (value : bool) -> Self { let value = value as inner :: BOOL ; Self { value } } # [doc = " Creates this from a raw boolean value."] # [doc = ""] # [doc = " Avoid this, and instead use `Bool` in the raw FFI signature."] # [inline] pub const fn from_raw (value : inner :: BOOL) -> Self { Self { value } } # [doc = " Retrieves the inner boolean type."] # [doc = ""] # [doc = " Avoid this, and instead use `Bool` in the raw FFI signature."] # [inline] pub const fn as_raw (self) -> inner :: BOOL { self . value } # [doc = " Returns `true` if `self` is [`NO`][Self::NO]."] # [doc = ""] # [doc = " You should prefer using [`as_bool`][Self::as_bool]."] # [inline] pub const fn is_false (self) -> bool { ! self . as_bool () } # [doc = " Returns `true` if `self` is not [`NO`][Self::NO]."] # [doc = ""] # [doc = " You should prefer using [`as_bool`][Self::as_bool]."] # [inline] pub const fn is_true (self) -> bool { self . as_bool () } # [doc = " Converts this into the [`bool`] equivalent."] # [inline] pub const fn as_bool (self) -> bool { self . value != (false as inner :: BOOL) } }
};
}
