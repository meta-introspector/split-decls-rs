// Generated macro for impl_55 (impl)
macro_rules! Depcrate_bytesimpl_55 {
() => {
// Module: crate::bytes
// Provides: {"impl_55"}
// Dependencies: {}
impl BytesRef { # [doc = " Constant value representing an empty byte slice."] pub const EMPTY : & 'static Self = Self :: new_unchecked (& []) ; # [doc = " Create a new [`BytesRef`], ensuring that the provided `slice` value"] # [doc = " is shorter than `Length::MAX`."] pub const fn new (slice : & [u8]) -> Result < & Self > { match Length :: new_usize (slice . len ()) { Ok (_) => Ok (Self :: new_unchecked (slice)) , Err (err) => Err (err) , } } # [doc = " Perform a raw conversion of a byte slice to `Self` without first performing a length check."] pub (crate) const fn new_unchecked (slice : & [u8]) -> & Self { # [allow (unsafe_code)] unsafe { & * (slice as * const [u8] as * const Self) } } # [doc = " Get a pointer to this [`BytesRef`]."] pub (crate) const fn as_ptr (& self) -> * const BytesRef { self as * const BytesRef } # [doc = " Borrow the inner byte slice"] pub const fn as_slice (& self) -> & [u8] { & self . 0 } # [doc = " Get the [`Length`] of this [`BytesRef`]."] pub fn len (& self) -> Length { debug_assert ! (u32 :: try_from (self . 0 . len ()) . is_ok ()) ; # [allow (clippy :: cast_possible_truncation)] Length :: new (self . 0 . len () as u32) } # [doc = " Is this [`BytesRef`] empty?"] pub const fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Get a prefix of a [`crate::bytes_ref::BytesRef`] of the given length."] pub fn prefix (& self , length : Length) -> Result < & Self > { let inner = self . as_slice () . get (.. usize :: try_from (length) ?) . ok_or_else (| | Error :: incomplete (self . len ())) ? ; Ok (Self :: new_unchecked (inner)) } }
};
}
