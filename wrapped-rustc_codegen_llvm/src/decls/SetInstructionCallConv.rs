macro_rules! deps {
    () => {
        CallConv!();
    };
}

macro_rules! SetInstructionCallConv {
    () => {
        deps!();
        pub (crate) fn SetInstructionCallConv (instr : & Value , cc : CallConv) { unsafe { LLVMSetInstructionCallConv (instr , cc as c_uint) ; } }
    };
}

SetInstructionCallConv!()