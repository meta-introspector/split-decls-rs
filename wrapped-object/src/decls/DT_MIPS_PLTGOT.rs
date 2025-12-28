macro_rules! DT_MIPS_PLTGOT {
    () => {
        # [doc = " The address of .got.plt in an executable using the new non-PIC ABI."] pub const DT_MIPS_PLTGOT : u32 = 0x7000_0032 ;
    };
}

DT_MIPS_PLTGOT!()