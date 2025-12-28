macro_rules! deps {
    () => {
        CodegenCx!();
        CreateAttrStringValue!();
    };
}

macro_rules! frame_pointer_type_attr {
    () => {
        deps!();
        pub (crate) fn frame_pointer_type_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { let mut fp = cx . sess () . target . frame_pointer ; let opts = & cx . sess () . opts ; if opts . unstable_opts . instrument_mcount { fp . ratchet (FramePointer :: Always) ; } fp . ratchet (opts . cg . force_frame_pointers) ; let attr_value = match fp { FramePointer :: Always => "all" , FramePointer :: NonLeaf => "non-leaf" , FramePointer :: MayOmit => return None , } ; Some (llvm :: CreateAttrStringValue (cx . llcx , "frame-pointer" , attr_value)) }
    };
}

frame_pointer_type_attr!()