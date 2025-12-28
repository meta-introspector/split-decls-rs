macro_rules! try_as_const_integral {
    () => {
        fn try_as_const_integral (v : & Value) -> Option < & ConstantInt > { unsafe { llvm :: LLVMIsAConstantInt (v) } }
    };
}

try_as_const_integral!();