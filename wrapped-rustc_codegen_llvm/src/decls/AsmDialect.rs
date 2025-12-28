macro_rules! AsmDialect {
    () => {
        # [doc = " Must match the layout of `LLVMInlineAsmDialect`."] # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum AsmDialect { Att , Intel , }
    };
}

AsmDialect!();