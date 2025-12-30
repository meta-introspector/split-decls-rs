// Generated macro for UnsupportedErrorKind (enum)
macro_rules! Depcrate_errorUnsupportedErrorKind {
() => {
// Module: crate::error
// Provides: {"UnsupportedErrorKind"}
// Dependencies: {}
# [doc = " Details what feature is not supported."] # [derive (Clone , Debug , Hash , PartialEq)] # [non_exhaustive] pub enum UnsupportedErrorKind { # [doc = " The required color type can not be handled."] Color (ExtendedColorType) , # [doc = " An image format is not supported."] Format (ImageFormatHint) , # [doc = " Some feature specified by string."] # [doc = " This is discouraged and is likely to get deprecated (but not removed)."] GenericFeature (String) , }
};
}
