// Generated macro for ColorError (enum)
macro_rules! Depcrate_colorColorError {
() => {
// Module: crate::color
// Provides: {"ColorError"}
// Dependencies: {}
# [doc = " An error that can occur when parsing color specifications."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ColorError { # [doc = " This occurs when an unrecognized output type is used."] UnrecognizedOutType (String) , # [doc = " This occurs when an unrecognized spec type is used."] UnrecognizedSpecType (String) , # [doc = " This occurs when an unrecognized color name is used."] UnrecognizedColor (String , String) , # [doc = " This occurs when an unrecognized style attribute is used."] UnrecognizedStyle (String) , # [doc = " This occurs when the format of a color specification is invalid."] InvalidFormat (String) , }
};
}
