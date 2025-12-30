// Generated macro for AnnotationSet (struct)
macro_rules! Depcrate_parsers_annotationsAnnotationSet {
() => {
// Module: crate::parsers::annotations
// Provides: {"AnnotationSet"}
// Dependencies: {}
# [doc = " Strictly a parsing intermediary for the checking the common annotation backing."] pub (crate) struct AnnotationSet < 'a , T : EncodingType > { pub (crate) tz : Option < TimeZoneAnnotation < 'a , T > > , pub (crate) calendar : Option < & 'a [T :: CodeUnit] > , }
};
}
