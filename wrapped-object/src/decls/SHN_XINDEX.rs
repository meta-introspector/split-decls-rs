macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHN_XINDEX {
    () => {
        deps!();
        # [doc = " Section index is in the `SHT_SYMTAB_SHNDX` section."] pub const SHN_XINDEX : u16 = 0xffff ;
    };
}

SHN_XINDEX!();