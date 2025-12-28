macro_rules! ELF_NOTE_GNU {
    () => {
        # [doc = " GNU entries in the note section have this name."] pub const ELF_NOTE_GNU : & [u8] = b"GNU" ;
    };
}

ELF_NOTE_GNU!()