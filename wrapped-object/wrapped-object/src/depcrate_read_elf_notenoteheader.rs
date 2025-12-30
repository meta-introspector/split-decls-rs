// Generated macro for NoteHeader (trait)
macro_rules! Depcrate_read_elf_noteNoteHeader {
() => {
// Module: crate::read::elf::note
// Provides: {"NoteHeader"}
// Dependencies: {}
# [doc = " A trait for generic access to [`elf::NoteHeader32`] and [`elf::NoteHeader64`]."] # [allow (missing_docs)] pub trait NoteHeader : Debug + Pod { type Endian : endian :: Endian ; fn n_namesz (& self , endian : Self :: Endian) -> u32 ; fn n_descsz (& self , endian : Self :: Endian) -> u32 ; fn n_type (& self , endian : Self :: Endian) -> u32 ; }
};
}
