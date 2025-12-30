// Generated macro for NoteCommand (struct)
macro_rules! Depcrate_machoNoteCommand {
() => {
// Module: crate::macho
// Provides: {"NoteCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct NoteCommand < E : Endian > { # [doc = " LC_NOTE"] pub cmd : U32 < E > , # [doc = " sizeof(struct NoteCommand)"] pub cmdsize : U32 < E > , # [doc = " owner name for this LC_NOTE"] pub data_owner : [u8 ; 16] , # [doc = " file offset of this data"] pub offset : U64 < E > , # [doc = " length of data region"] pub size : U64 < E > , }
};
}
