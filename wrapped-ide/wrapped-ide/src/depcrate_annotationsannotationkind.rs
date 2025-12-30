// Generated macro for AnnotationKind (enum)
macro_rules! Depcrate_annotationsAnnotationKind {
() => {
// Module: crate::annotations
// Provides: {"AnnotationKind"}
// Dependencies: {}
# [derive (Debug , Hash , PartialEq , Eq)] pub enum AnnotationKind { Runnable (Runnable) , HasImpls { pos : FilePosition , data : Option < Vec < NavigationTarget > > } , HasReferences { pos : FilePosition , data : Option < Vec < FileRange > > } , }
};
}
