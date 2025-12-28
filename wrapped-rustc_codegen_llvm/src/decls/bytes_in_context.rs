macro_rules! bytes_in_context {
    () => {
        pub (crate) fn bytes_in_context < 'll > (llcx : & 'll llvm :: Context , bytes : & [u8]) -> & 'll Value { unsafe { let ptr = bytes . as_ptr () as * const c_char ; llvm :: LLVMConstStringInContext2 (llcx , ptr , bytes . len () , TRUE) } }
    };
}

bytes_in_context!()