macro_rules! deps {
    () => {
        DIB!();
        CodegenCx!();
    };
}

macro_rules! create_basic_type {
    () => {
        deps!();
        fn create_basic_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , name : & str , size : Size , encoding : u32 ,) -> & 'll DIBasicType { unsafe { llvm :: LLVMRustDIBuilderCreateBasicType (DIB (cx) , name . as_c_char_ptr () , name . len () , size . bits () , encoding ,) } }
    };
}

create_basic_type!()