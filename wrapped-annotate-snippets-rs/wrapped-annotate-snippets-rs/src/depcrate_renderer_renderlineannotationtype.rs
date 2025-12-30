// Generated macro for LineAnnotationType (enum)
macro_rules! Depcrate_renderer_renderLineAnnotationType {
() => {
// Module: crate::renderer::render
// Provides: {"LineAnnotationType"}
// Dependencies: {}
# [derive (Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) enum LineAnnotationType { # [doc = " Annotation under a single line of code"] Singleline , # [doc = " Annotation marking the first character of a fully shown multiline span"] MultilineStart (usize) , # [doc = " Annotation marking the last character of a fully shown multiline span"] MultilineEnd (usize) , # [doc = " Line at the left enclosing the lines of a fully shown multiline span"] MultilineLine (usize) , }
};
}
