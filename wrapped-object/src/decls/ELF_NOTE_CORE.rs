macro_rules! deps {
    () => {
        Note!();
    };
}

macro_rules! ELF_NOTE_CORE {
    () => {
        deps!();
        # [doc = " Note name for core files."] pub const ELF_NOTE_CORE : & [u8] = b"CORE" ;
    };
}

ELF_NOTE_CORE!();