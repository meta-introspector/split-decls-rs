// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_util_borrowimpl_1176 {
() => {
// Module: crate::util::borrow
// Provides: {"impl_1176"}
// Dependencies: {}
impl < 'a > StringCow < 'a > { # [doc = " Returns this `String` or `&str` as a `&str`."] # [doc = ""] # [doc = " Like `std::borrow::Cow`, the lifetime of the string slice returned"] # [doc = " is tied to `StringCow`, and _not_ the original lifetime of the string"] # [doc = " slice."] pub (crate) fn as_str < 's > (& 's self) -> & 's str { match * self { # [cfg (feature = "alloc")] StringCow :: Owned (ref s) => s , StringCow :: Borrowed (s) => s , } } # [doc = " Converts this cow into an \"owned\" variant, copying if necessary."] # [doc = ""] # [doc = " If this cow is already an \"owned\" variant, then this is a no-op."] # [cfg (feature = "alloc")] pub (crate) fn into_owned (self) -> StringCow < 'static > { use alloc :: string :: ToString ; match self { StringCow :: Owned (string) => StringCow :: Owned (string) , StringCow :: Borrowed (string) => { StringCow :: Owned (string . to_string ()) } } } }
};
}
