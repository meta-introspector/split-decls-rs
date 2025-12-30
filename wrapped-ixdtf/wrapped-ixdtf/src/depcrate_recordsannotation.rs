// Generated macro for Annotation (struct)
macro_rules! Depcrate_recordsAnnotation {
() => {
// Module: crate::records
// Provides: {"Annotation"}
// Dependencies: {}
# [non_exhaustive] # [derive (Debug , Clone , PartialEq)] # [doc = " A record of an annotation."] pub struct Annotation < 'a , T : EncodingType > { # [doc = " Whether this annotation is flagged as critical"] pub critical : bool , # [doc = " The parsed key value of the annotation"] pub key : & 'a [T :: CodeUnit] , # [doc = " The parsed value of the annotation"] pub value : & 'a [T :: CodeUnit] , }
};
}
