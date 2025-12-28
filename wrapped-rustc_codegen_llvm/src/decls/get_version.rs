macro_rules! get_version {
    () => {
        pub (crate) fn get_version () -> (u32 , u32 , u32) { unsafe { (llvm :: LLVMRustVersionMajor () , llvm :: LLVMRustVersionMinor () , llvm :: LLVMRustVersionPatch ()) } }
    };
}

get_version!()