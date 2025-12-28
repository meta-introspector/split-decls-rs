macro_rules! deps {
    () => {
        Note!();
    };
}

macro_rules! ELF_NOTE_LINUX {
    () => {
        deps!();
        # [doc = " Note name for linux core files."] # [doc = ""] # [doc = " Notes in linux core files may also use `ELF_NOTE_CORE`."] pub const ELF_NOTE_LINUX : & [u8] = b"LINUX" ;
    };
}

ELF_NOTE_LINUX!()