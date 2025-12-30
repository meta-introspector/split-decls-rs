// Generated macro for Ident (struct)
macro_rules! Depcrate_identIdent {
() => {
// Module: crate::ident
// Provides: {"Ident"}
// Dependencies: {}
# [doc = " Algorithm or parameter identifier."] # [doc = ""] # [doc = " This type encompasses both the \"function symbolic name\" and \"parameter name\""] # [doc = " use cases as described in the [PHC string format specification][1]."] # [doc = ""] # [doc = " # Constraints"] # [doc = " - ASCII-encoded string consisting of the characters `[a-z0-9-]`"] # [doc = "   (lowercase letters, digits, and the minus sign)"] # [doc = " - Minimum length: 1 ASCII character (i.e. 1-byte)"] # [doc = " - Maximum length: 32 ASCII characters (i.e. 32-bytes)"] # [doc = ""] # [doc = " [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md"] # [derive (Copy , Clone , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct Ident < 'a > (& 'a str) ;
};
}
