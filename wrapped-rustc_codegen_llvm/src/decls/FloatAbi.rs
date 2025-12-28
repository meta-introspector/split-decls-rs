macro_rules! FloatAbi {
    () => {
        # [doc = " LLVMRustFloatABI"] # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum FloatAbi { Default , Soft , Hard , }
    };
}

FloatAbi!()