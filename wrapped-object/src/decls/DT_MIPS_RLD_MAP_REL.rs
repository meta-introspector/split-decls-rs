macro_rules! DT_MIPS_RLD_MAP_REL {
    () => {
        # [doc = " An alternative description of the classic MIPS RLD_MAP that is usable in a PIE as it stores a relative offset from the address of the tag rather than an absolute address."] pub const DT_MIPS_RLD_MAP_REL : u32 = 0x7000_0035 ;
    };
}

DT_MIPS_RLD_MAP_REL!();