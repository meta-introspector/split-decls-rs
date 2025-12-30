// Generated macro for macro_16 (macro)
macro_rules! Depcrate_binarymacro_16 {
() => {
// Module: crate::binary
// Provides: {"macro_16"}
// Dependencies: {}
primitive_enum ! (u8 , # [doc = " A family of character sets used to represent the characters of key strings."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum CharsetFamily { # [doc = " The ASCII family of character sets, such as ASCII, latin1, and"] # [doc = " UTF-8."] Ascii = 0 , # [doc = " The EBCDIC family of character sets, such as EBCDIC and UTF-EBCDIC."] # [doc = ""] # [doc = " The EBCDIC family is currently unsupported by this crate both for"] # [doc = " serialization and deserialization of binary bundles."] Ebcdic = 1 , }) ;
};
}
