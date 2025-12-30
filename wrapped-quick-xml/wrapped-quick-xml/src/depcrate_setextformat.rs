// Generated macro for TextFormat (enum)
macro_rules! Depcrate_seTextFormat {
() => {
// Module: crate::se
// Provides: {"TextFormat"}
// Dependencies: {}
# [doc = " Defines the format for text content serialization"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum TextFormat { # [doc = " Serialize as regular text content with escaping"] Text , # [doc = " Serialize as CDATA section without escaping"] CData , }
};
}
