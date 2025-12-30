// Generated macro for impl_1494 (impl)
macro_rules! Depcrate_stringimpl_1494 {
() => {
// Module: crate::string
// Provides: {"impl_1494"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_from_string_ref" , since = "1.28.0")] impl < 'a > From < & 'a String > for Cow < 'a , str > { # [doc = " Converts a [`String`] reference into a [`Borrowed`] variant."] # [doc = " No heap allocation is performed, and the string"] # [doc = " is not copied."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::borrow::Cow;"] # [doc = " let s = \"eggplant\".to_string();"] # [doc = " assert_eq!(Cow::from(&s), Cow::Borrowed(\"eggplant\"));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Borrowed`]: crate::borrow::Cow::Borrowed \"borrow::Cow::Borrowed\""] # [inline] fn from (s : & 'a String) -> Cow < 'a , str > { Cow :: Borrowed (s . as_str ()) } }
};
}
