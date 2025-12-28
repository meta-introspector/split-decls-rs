macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! ELFOSABI_LINUX {
    () => {
        deps!();
        # [doc = " Object uses GNU ELF extensions."] # [doc = ""] # [doc = " Compatibility alias."] pub const ELFOSABI_LINUX : u8 = ELFOSABI_GNU ;
    };
}

ELFOSABI_LINUX!();