// Generated macro for MultilineAnnotation (struct)
macro_rules! Depcrate_renderer_source_mapMultilineAnnotation {
() => {
// Module: crate::renderer::source_map
// Provides: {"MultilineAnnotation"}
// Dependencies: {}
# [derive (Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) struct MultilineAnnotation < 'a > { pub depth : usize , pub start : Loc , pub end : Loc , pub kind : AnnotationKind , pub label : Option < Cow < 'a , str > > , pub overlaps_exactly : bool , pub highlight_source : bool , }
};
}
