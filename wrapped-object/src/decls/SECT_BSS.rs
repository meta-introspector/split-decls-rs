macro_rules! SECT_BSS {
    () => {
        # [doc = " the real uninitialized data section no padding"] pub const SECT_BSS : & str = "__bss" ;
    };
}

SECT_BSS!()