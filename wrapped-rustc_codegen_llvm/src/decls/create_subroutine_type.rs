macro_rules! deps {
    () => {
        DIB!();
        CodegenCx!();
    };
}

macro_rules! create_subroutine_type {
    () => {
        deps!();
        pub (super) fn create_subroutine_type < 'll > (cx : & CodegenCx < 'll , '_ > , signature : & 'll DICompositeType ,) -> & 'll DICompositeType { unsafe { llvm :: LLVMRustDIBuilderCreateSubroutineType (DIB (cx) , signature) } }
    };
}

create_subroutine_type!();