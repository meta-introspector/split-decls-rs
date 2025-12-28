macro_rules! LLVMRustResult {
    () => {
        # [derive (Copy , Clone , PartialEq)] # [repr (C)] # [allow (dead_code)] pub (crate) enum LLVMRustResult { Success , Failure , }
    };
}

LLVMRustResult!()