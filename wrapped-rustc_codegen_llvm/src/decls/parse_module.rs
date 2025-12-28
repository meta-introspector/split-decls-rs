macro_rules! deps {
    () => {
        LlvmError!();
    };
}

macro_rules! parse_module {
    () => {
        deps!();
        pub (crate) fn parse_module < 'a > (cx : & 'a llvm :: Context , name : & CStr , data : & [u8] , dcx : DiagCtxtHandle < '_ > ,) -> & 'a llvm :: Module { unsafe { llvm :: LLVMRustParseBitcodeForLTO (cx , data . as_ptr () , data . len () , name . as_ptr ()) . unwrap_or_else (| | write :: llvm_err (dcx , LlvmError :: ParseBitcode)) } }
    };
}

parse_module!()