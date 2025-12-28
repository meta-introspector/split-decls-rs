macro_rules! deps {
    () => {
        Endian!();
        U32!();
        U64!();
    };
}

macro_rules! NoteCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct NoteCommand < E : Endian > { # [doc = " LC_NOTE"] pub cmd : U32 < E > , # [doc = " sizeof(struct NoteCommand)"] pub cmdsize : U32 < E > , # [doc = " owner name for this LC_NOTE"] pub data_owner : [u8 ; 16] , # [doc = " file offset of this data"] pub offset : U64 < E > , # [doc = " length of data region"] pub size : U64 < E > , }
    };
}

NoteCommand!();