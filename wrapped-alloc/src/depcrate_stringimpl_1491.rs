// Generated macro for impl_1491 (impl)
macro_rules! Depcrate_stringimpl_1491 {
() => {
// Module: crate::string
// Provides: {"impl_1491"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "string_from_cow_str" , since = "1.14.0")] impl < 'a > From < Cow < 'a , str > > for String { # [doc = " Converts a clone-on-write string to an owned"] # [doc = " instance of [`String`]."] # [doc = ""] # [doc = " This extracts the owned string,"] # [doc = " clones the string if it is not already owned."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::borrow::Cow;"] # [doc = " // If the string is not owned..."] # [doc = " let cow: Cow<'_, str> = Cow::Borrowed(\"eggplant\");"] # [doc = " // It will allocate on the heap and copy the string."] # [doc = " let owned: String = String::from(cow);"] # [doc = " assert_eq!(&owned[..], \"eggplant\");"] # [doc = " ```"] fn from (s : Cow < 'a , str >) -> String { s . into_owned () } }
};
}
