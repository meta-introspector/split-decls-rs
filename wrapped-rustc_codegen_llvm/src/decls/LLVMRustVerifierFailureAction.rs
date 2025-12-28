macro_rules! LLVMRustVerifierFailureAction {
    () => {
        # [repr (C)] # [derive (Copy , Clone , PartialEq)] pub (crate) enum LLVMRustVerifierFailureAction { LLVMAbortProcessAction = 0 , LLVMPrintMessageAction = 1 , LLVMReturnStatusAction = 2 , }
    };
}

LLVMRustVerifierFailureAction!()