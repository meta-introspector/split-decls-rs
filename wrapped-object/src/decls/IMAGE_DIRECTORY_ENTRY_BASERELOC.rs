macro_rules! deps {
    () => {
        Table!();
        Relocation!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_BASERELOC {
    () => {
        deps!();
        # [doc = " Base Relocation Table"] pub const IMAGE_DIRECTORY_ENTRY_BASERELOC : usize = 5 ;
    };
}

IMAGE_DIRECTORY_ENTRY_BASERELOC!();