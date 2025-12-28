macro_rules! deps {
    () => {
        FileHeader!();
        NoteHeader!();
    };
}

macro_rules! Note {
    () => {
        deps!();
        # [doc = " A parsed [`NoteHeader`]."] # [derive (Debug)] pub struct Note < 'data , Elf > where Elf : FileHeader , { header : & 'data Elf :: NoteHeader , name : & 'data [u8] , desc : & 'data [u8] , }
    };
}

Note!();