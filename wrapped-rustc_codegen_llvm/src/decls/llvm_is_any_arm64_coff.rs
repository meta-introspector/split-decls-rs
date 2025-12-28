macro_rules! llvm_is_any_arm64_coff {
    () => {
        fn llvm_is_any_arm64_coff (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIsAnyArm64Coff (buf . as_ptr () , buf . len ()) } }
    };
}

llvm_is_any_arm64_coff!()