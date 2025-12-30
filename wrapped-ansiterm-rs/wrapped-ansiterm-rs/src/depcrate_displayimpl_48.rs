// Generated macro for impl_48 (impl)
macro_rules! Depcrate_displayimpl_48 {
() => {
// Module: crate::display
// Provides: {"impl_48"}
// Dependencies: {}
# [doc = " Cloning an `ANSIGenericString` will clone its underlying string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ansiterm::ANSIString;"] # [doc = ""] # [doc = " let plain_string = ANSIString::from(\"a plain string\");"] # [doc = " let clone_string = plain_string.clone();"] # [doc = " assert_eq!(clone_string, plain_string);"] # [doc = " ```"] impl < 'a , S : 'a + ToOwned + ? Sized > Clone for ANSIGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug { fn clone (& self) -> ANSIGenericString < 'a , S > { ANSIGenericString { style : self . style , string : self . string . clone () , } } }
};
}
