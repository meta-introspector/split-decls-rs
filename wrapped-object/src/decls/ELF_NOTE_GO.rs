macro_rules! ELF_NOTE_GO {
    () => {
        # [doc = " Go entries in the note section have this name."] pub const ELF_NOTE_GO : & [u8] = b"Go" ;
    };
}

ELF_NOTE_GO!()