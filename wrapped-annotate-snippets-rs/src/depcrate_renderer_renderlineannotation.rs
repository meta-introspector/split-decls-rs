// Generated macro for LineAnnotation (struct)
macro_rules! Depcrate_renderer_renderLineAnnotation {
() => {
// Module: crate::renderer::render
// Provides: {"LineAnnotation"}
// Dependencies: {}
# [derive (Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) struct LineAnnotation < 'a > { # [doc = " Start column."] # [doc = " Note that it is important that this field goes"] # [doc = " first, so that when we sort, we sort orderings by start"] # [doc = " column."] pub start : Loc , # [doc = " End column within the line (exclusive)"] pub end : Loc , # [doc = " level"] pub kind : AnnotationKind , # [doc = " Optional label to display adjacent to the annotation."] pub label : Option < Cow < 'a , str > > , # [doc = " Is this a single line, multiline or multiline span minimized down to a"] # [doc = " smaller span."] pub annotation_type : LineAnnotationType , # [doc = " Whether the source code should be highlighted"] pub highlight_source : bool , }
};
}
