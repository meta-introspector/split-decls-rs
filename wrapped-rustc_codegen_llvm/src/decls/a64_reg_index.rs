macro_rules! a64_reg_index {
    () => {
        # [doc = " If the register is an AArch64 integer register then return its index."] fn a64_reg_index (reg : InlineAsmReg) -> Option < u32 > { match reg { InlineAsmReg :: AArch64 (r) => r . reg_index () , _ => None , } }
    };
}

a64_reg_index!()