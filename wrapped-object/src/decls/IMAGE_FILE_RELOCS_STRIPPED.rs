macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! IMAGE_FILE_RELOCS_STRIPPED {
    () => {
        deps!();
        # [doc = " Relocation info stripped from file."] pub const IMAGE_FILE_RELOCS_STRIPPED : u16 = 0x0001 ;
    };
}

IMAGE_FILE_RELOCS_STRIPPED!();