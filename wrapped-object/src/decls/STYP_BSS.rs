macro_rules! STYP_BSS {
    () => {
        # [doc = " Specifies an uninitialized data section. A section header of this type"] # [doc = " defines the uninitialized data of a program."] pub const STYP_BSS : u16 = 0x80 ;
    };
}

STYP_BSS!();