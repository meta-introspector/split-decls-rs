// Generated macro for Attribute (enum)
macro_rules! Depcrate_parseAttribute {
() => {
// Module: crate::parse
// Provides: {"Attribute"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone)] pub (crate) enum Attribute { Attr (Ident) , Tagged (Ident , Vec < Pat >) , Type (Ident , Box < syn :: Type >) , }
};
}
