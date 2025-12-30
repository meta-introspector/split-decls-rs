// Generated macro for frame_pointer_type_attr (function)
macro_rules! Depcrate_attributesframe_pointer_type_attr {
() => {
// Module: crate::attributes
// Provides: {"frame_pointer_type_attr"}
// Dependencies: {}
pub (crate) fn frame_pointer_type_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { let mut fp = cx . sess () . target . frame_pointer ; let opts = & cx . sess () . opts ; if opts . unstable_opts . instrument_mcount { fp . ratchet (FramePointer :: Always) ; } fp . ratchet (opts . cg . force_frame_pointers) ; let attr_value = match fp { FramePointer :: Always => "all" , FramePointer :: NonLeaf => "non-leaf" , FramePointer :: MayOmit => return None , } ; Some (llvm :: CreateAttrStringValue (cx . llcx , "frame-pointer" , attr_value)) }
};
}
