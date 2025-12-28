macro_rules! R_IA64_GPREL64I {
    () => {
        # [doc = " @gprel(sym + add), mov imm64"] pub const R_IA64_GPREL64I : u32 = 0x2b ;
    };
}

R_IA64_GPREL64I!();