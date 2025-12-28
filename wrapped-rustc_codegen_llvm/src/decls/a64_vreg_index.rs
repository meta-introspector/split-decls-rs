macro_rules! a64_vreg_index {
    () => {
        # [doc = " If the register is an AArch64 vector register then return its index."] fn a64_vreg_index (reg : InlineAsmReg) -> Option < u32 > { match reg { InlineAsmReg :: AArch64 (reg) => reg . vreg_index () , _ => None , } }
    };
}

a64_vreg_index!()