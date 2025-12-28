macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LtoBitcodeFromRlib {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_lto_bitcode_from_rlib)] pub (crate) struct LtoBitcodeFromRlib { pub err : String , }
    };
}

LtoBitcodeFromRlib!()