// Generated macro for impl_1493 (impl)
macro_rules! Depcrate_stringimpl_1493 {
() => {
// Module: crate::string
// Provides: {"impl_1493"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > From < String > for Cow < 'a , str > { # [doc = " Converts a [`String`] into an [`Owned`] variant."] # [doc = " No heap allocation is performed, and the string"] # [doc = " is not copied."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::borrow::Cow;"] # [doc = " let s = \"eggplant\".to_string();"] # [doc = " let s2 = \"eggplant\".to_string();"] # [doc = " assert_eq!(Cow::from(s), Cow::<'static, str>::Owned(s2));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Owned`]: crate::borrow::Cow::Owned \"borrow::Cow::Owned\""] # [inline] fn from (s : String) -> Cow < 'a , str > { Cow :: Owned (s) } }
};
}
