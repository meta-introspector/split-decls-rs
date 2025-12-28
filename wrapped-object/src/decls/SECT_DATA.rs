macro_rules! SECT_DATA {
    () => {
        # [doc = " the real initialized data section no padding, no bss overlap"] pub const SECT_DATA : & str = "__data" ;
    };
}

SECT_DATA!();