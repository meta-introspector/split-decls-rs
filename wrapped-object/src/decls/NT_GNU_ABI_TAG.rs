macro_rules! NT_GNU_ABI_TAG {
    () => {
        # [doc = " ABI information."] # [doc = ""] # [doc = " The descriptor consists of words:"] # [doc = " - word 0: OS descriptor"] # [doc = " - word 1: major version of the ABI"] # [doc = " - word 2: minor version of the ABI"] # [doc = " - word 3: subminor version of the ABI"] pub const NT_GNU_ABI_TAG : u32 = 1 ;
    };
}

NT_GNU_ABI_TAG!()