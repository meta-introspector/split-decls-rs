// Generated macro for NoteHeader32 (struct)
macro_rules! Depcrate_elfNoteHeader32 {
() => {
// Module: crate::elf
// Provides: {"NoteHeader32"}
// Dependencies: {}
# [doc = " Note section entry header."] # [doc = ""] # [doc = " A note consists of a header followed by a variable length name and descriptor."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct NoteHeader32 < E : Endian > { # [doc = " Length of the note's name."] # [doc = ""] # [doc = " Some known names are defined by the `ELF_NOTE_*` constants."] pub n_namesz : U32 < E > , # [doc = " Length of the note's descriptor."] # [doc = ""] # [doc = " The content of the descriptor depends on the note name and type."] pub n_descsz : U32 < E > , # [doc = " Type of the note."] # [doc = ""] # [doc = " One of the `NT_*` constants. The note name determines which"] # [doc = " `NT_*` constants are valid."] pub n_type : U32 < E > , }
};
}
