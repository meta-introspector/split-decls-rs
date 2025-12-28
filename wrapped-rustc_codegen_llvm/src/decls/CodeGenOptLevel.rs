macro_rules! CodeGenOptLevel {
    () => {
        # [doc = " LLVMRustCodeGenOptLevel"] # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum CodeGenOptLevel { None , Less , Default , Aggressive , }
    };
}

CodeGenOptLevel!();