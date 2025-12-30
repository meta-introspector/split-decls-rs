// Generated macro for Element (enum)
macro_rules! Depcrate_snippetElement {
() => {
// Module: crate::snippet
// Provides: {"Element"}
// Dependencies: {}
# [doc = " A section of content within a [`Group`]"] # [derive (Clone , Debug)] # [non_exhaustive] pub enum Element < 'a > { Message (Message < 'a >) , Cause (Snippet < 'a , Annotation < 'a > >) , Suggestion (Snippet < 'a , Patch < 'a > >) , Origin (Origin < 'a >) , Padding (Padding) , }
};
}
