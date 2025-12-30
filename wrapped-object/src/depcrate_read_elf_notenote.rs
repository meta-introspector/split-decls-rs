// Generated macro for Note (struct)
macro_rules! Depcrate_read_elf_noteNote {
() => {
// Module: crate::read::elf::note
// Provides: {"Note"}
// Dependencies: {}
# [doc = " A parsed [`NoteHeader`]."] # [derive (Debug)] pub struct Note < 'data , Elf > where Elf : FileHeader , { header : & 'data Elf :: NoteHeader , name : & 'data [u8] , desc : & 'data [u8] , }
};
}
