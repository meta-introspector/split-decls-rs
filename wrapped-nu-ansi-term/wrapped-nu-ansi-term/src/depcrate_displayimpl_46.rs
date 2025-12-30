// Generated macro for impl_46 (impl)
macro_rules! Depcrate_displayimpl_46 {
() => {
// Module: crate::display
// Provides: {"impl_46"}
// Dependencies: {}
# [doc = " Cloning an `AnsiGenericString` will clone its underlying string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::AnsiString;"] # [doc = ""] # [doc = " let plain_string = AnsiString::from(\"a plain string\");"] # [doc = " let clone_string = plain_string.clone();"] # [doc = " assert_eq!(clone_string, plain_string);"] # [doc = " ```"] impl < 'a , S : 'a + ToOwned + ? Sized > Clone for AnsiGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , { fn clone (& self) -> AnsiGenericString < 'a , S > { AnsiGenericString { style : self . style , string : self . string . clone () , oscontrol : self . oscontrol . clone () , } } }
};
}
