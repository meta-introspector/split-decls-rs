macro_rules! llvm_is_64_bit_object_file {
    () => {
        fn llvm_is_64_bit_object_file (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIs64BitSymbolicFile (buf . as_ptr () , buf . len ()) } }
    };
}

llvm_is_64_bit_object_file!();