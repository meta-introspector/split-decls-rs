macro_rules! deps {
    () => {
        FloatAbi!();
    };
}

macro_rules! to_llvm_float_abi {
    () => {
        deps!();
        fn to_llvm_float_abi (float_abi : Option < FloatAbi >) -> llvm :: FloatAbi { match float_abi { None => llvm :: FloatAbi :: Default , Some (FloatAbi :: Soft) => llvm :: FloatAbi :: Soft , Some (FloatAbi :: Hard) => llvm :: FloatAbi :: Hard , } }
    };
}

to_llvm_float_abi!();