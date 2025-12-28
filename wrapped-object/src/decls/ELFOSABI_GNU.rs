macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! ELFOSABI_GNU {
    () => {
        deps!();
        # [doc = " Object uses GNU ELF extensions."] pub const ELFOSABI_GNU : u8 = 3 ;
    };
}

ELFOSABI_GNU!();