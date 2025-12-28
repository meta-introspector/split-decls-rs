macro_rules! ELF_NOTE_SOLARIS {
    () => {
        # [doc = " Solaris entries in the note section have this name."] pub const ELF_NOTE_SOLARIS : & [u8] = b"SUNW Solaris" ;
    };
}

ELF_NOTE_SOLARIS!();