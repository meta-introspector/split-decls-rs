// Generated macro for impl_225 (impl)
macro_rules! Depcrate_stringimpl_225 {
() => {
// Module: crate::string
// Provides: {"impl_225"}
// Dependencies: {}
impl StringRef { # [doc = " Create a new [`StringRef`], ensuring that the byte representation of"] # [doc = " the provided `str` value is shorter than `Length::max()`."] pub const fn new (s : & str) -> Result < & Self > { match Length :: new_usize (s . len ()) { Ok (_) => Ok (Self :: new_unchecked (s)) , Err (err) => Err (err) , } } # [doc = " Perform a raw conversion of a `str` to `Self` without first performing a length check."] pub (crate) const fn new_unchecked (s : & str) -> & Self { # [allow (unsafe_code)] unsafe { & * (s as * const str as * const Self) } } # [doc = " Parse a [`StringRef`] from UTF-8 encoded bytes."] pub fn from_bytes (bytes : & [u8]) -> Result < & Self > { Self :: new (str :: from_utf8 (bytes) ?) } # [doc = " Borrow the inner `str`."] pub fn as_str (& self) -> & str { & self . 0 } # [doc = " Borrow the inner byte slice."] pub fn as_bytes (& self) -> & [u8] { self . 0 . as_bytes () } # [doc = " Get the [`Length`] of this [`StringRef`]."] pub fn len (& self) -> Length { debug_assert ! (u32 :: try_from (self . 0 . len ()) . is_ok ()) ; # [allow (clippy :: cast_possible_truncation)] Length :: new (self . 0 . len () as u32) } # [doc = " Is this [`StringRef`] empty?"] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } }
};
}
