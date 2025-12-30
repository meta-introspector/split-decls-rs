// Generated macro for StringCow (enum)
macro_rules! Depcrate_util_borrowStringCow {
() => {
// Module: crate::util::borrow
// Provides: {"StringCow"}
// Dependencies: {}
# [doc = " A `Cow`, but can be used in core-only mode."] # [doc = ""] # [doc = " In core-only, the `Owned` variant doesn't exist."] # [derive (Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub (crate) enum StringCow < 'a > { # [cfg (feature = "alloc")] Owned (alloc :: string :: String) , Borrowed (& 'a str) , }
};
}
