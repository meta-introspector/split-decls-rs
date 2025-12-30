// Generated macro for PropertyKind (enum)
macro_rules! Depcrate_methodPropertyKind {
() => {
// Module: crate::method
// Provides: {"PropertyKind"}
// Dependencies: {}
# [doc = " <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProperties.html>"] # [doc = " <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/EncapsulatingData/EncapsulatingData.html#//apple_ref/doc/uid/TP40011210-CH5-SW3>."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy , Default)] pub enum PropertyKind { # [doc = " By default, properties are retained."] # [default] Normal , # [doc = " The object is copied into the property when set. These are retained."] Copy , # [doc = " The object is weakly referenced by the property."] Weak , # [doc = " Whether the property is marked as not internally retained,"] # [doc = " neither strongly nor weakly."] UnsafeRetained , }
};
}
