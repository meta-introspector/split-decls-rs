macro_rules! EF_AVR_LINKRELAX_PREPARED {
    () => {
        # [doc = " If set, it is assumed that the elf file uses local symbols as reference"] # [doc = " for the relocations so that linker relaxation is possible."] pub const EF_AVR_LINKRELAX_PREPARED : u32 = 0x80 ;
    };
}

EF_AVR_LINKRELAX_PREPARED!();