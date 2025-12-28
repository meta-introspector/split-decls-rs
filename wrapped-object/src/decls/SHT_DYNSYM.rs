macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! SHT_DYNSYM {
    () => {
        deps!();
        # [doc = " Dynamic linker symbol table."] pub const SHT_DYNSYM : u32 = 11 ;
    };
}

SHT_DYNSYM!();