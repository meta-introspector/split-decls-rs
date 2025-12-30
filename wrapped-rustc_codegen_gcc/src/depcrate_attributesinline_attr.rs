// Generated macro for inline_attr (function)
macro_rules! Depcrate_attributesinline_attr {
() => {
// Module: crate::attributes
// Provides: {"inline_attr"}
// Dependencies: {}
# [doc = " Get GCC attribute for the provided inline heuristic, attached to `instance`."] # [cfg (feature = "master")] # [inline] fn inline_attr < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , inline : InlineAttr , instance : ty :: Instance < 'tcx > ,) -> Option < FnAttribute < 'gcc > > { match inline { InlineAttr :: Always => { if recursively_inline (cx , instance) { Some (FnAttribute :: Inline) } else { Some (FnAttribute :: AlwaysInline) } } InlineAttr :: Hint => Some (FnAttribute :: Inline) , InlineAttr :: Force { .. } => Some (FnAttribute :: AlwaysInline) , InlineAttr :: Never => { if cx . sess () . target . arch != "amdgpu" { Some (FnAttribute :: NoInline) } else { None } } InlineAttr :: None => None , } }
};
}
