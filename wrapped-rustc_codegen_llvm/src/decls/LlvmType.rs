macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! LlvmType {
    () => {
        deps!();
        pub (crate) trait LlvmType { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type ; }
    };
}

LlvmType!()