macro_rules! llvm_is_ec_object_file {
    () => {
        fn llvm_is_ec_object_file (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIsECObject (buf . as_ptr () , buf . len ()) } }
    };
}

llvm_is_ec_object_file!();