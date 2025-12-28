macro_rules! create_DIArray {
    () => {
        # [allow (non_snake_case)] pub (crate) fn create_DIArray < 'll > (builder : & DIBuilder < 'll > , arr : & [Option < & 'll DIDescriptor >] ,) -> & 'll DIArray { unsafe { llvm :: LLVMRustDIBuilderGetOrCreateArray (builder , arr . as_ptr () , arr . len () as u32) } }
    };
}

create_DIArray!();