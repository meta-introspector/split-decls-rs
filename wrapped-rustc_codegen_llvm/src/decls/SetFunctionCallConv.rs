macro_rules! deps {
    () => {
        CallConv!();
    };
}

macro_rules! SetFunctionCallConv {
    () => {
        deps!();
        pub (crate) fn SetFunctionCallConv (fn_ : & Value , cc : CallConv) { unsafe { LLVMSetFunctionCallConv (fn_ , cc as c_uint) ; } }
    };
}

SetFunctionCallConv!()