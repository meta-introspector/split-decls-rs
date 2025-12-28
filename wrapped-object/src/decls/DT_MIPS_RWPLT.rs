macro_rules! DT_MIPS_RWPLT {
    () => {
        # [doc = " The base of the PLT in an executable using the new non-PIC ABI if that PLT is writable.  For a non-writable PLT, this is omitted or has a zero value."] pub const DT_MIPS_RWPLT : u32 = 0x7000_0034 ;
    };
}

DT_MIPS_RWPLT!();