macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        CodegenCx!();
        CreateAttrString!();
        SmallVec!();
    };
}

macro_rules! instrument_function_attr {
    () => {
        deps!();
        # [doc = " Tell LLVM what instrument function to insert."] # [inline] fn instrument_function_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> SmallVec < [& 'll Attribute ; 4] > { let mut attrs = SmallVec :: new () ; if cx . sess () . opts . unstable_opts . instrument_mcount { let mcount_name = match & cx . sess () . target . llvm_mcount_intrinsic { Some (llvm_mcount_intrinsic) => llvm_mcount_intrinsic . as_ref () , None => cx . sess () . target . mcount . as_ref () , } ; attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "instrument-function-entry-inlined" , mcount_name ,)) ; } if let Some (options) = & cx . sess () . opts . unstable_opts . instrument_xray { if options . always { attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "function-instrument" , "xray-always")) ; } if options . never { attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "function-instrument" , "xray-never")) ; } if options . ignore_loops { attrs . push (llvm :: CreateAttrString (cx . llcx , "xray-ignore-loops")) ; } let threshold = options . instruction_threshold . unwrap_or (200) ; attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "xray-instruction-threshold" , & threshold . to_string () ,)) ; if options . skip_entry { attrs . push (llvm :: CreateAttrString (cx . llcx , "xray-skip-entry")) ; } if options . skip_exit { attrs . push (llvm :: CreateAttrString (cx . llcx , "xray-skip-exit")) ; } } attrs }
    };
}

instrument_function_attr!();