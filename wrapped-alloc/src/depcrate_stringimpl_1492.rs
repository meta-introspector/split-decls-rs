// Generated macro for impl_1492 (impl)
macro_rules! Depcrate_stringimpl_1492 {
() => {
// Module: crate::string
// Provides: {"impl_1492"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > From < & 'a str > for Cow < 'a , str > { # [doc = " Converts a string slice into a [`Borrowed`] variant."] # [doc = " No heap allocation is performed, and the string"] # [doc = " is not copied."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::borrow::Cow;"] # [doc = " assert_eq!(Cow::from(\"eggplant\"), Cow::Borrowed(\"eggplant\"));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Borrowed`]: crate::borrow::Cow::Borrowed \"borrow::Cow::Borrowed\""] # [inline] fn from (s : & 'a str) -> Cow < 'a , str > { Cow :: Borrowed (s) } }
};
}
