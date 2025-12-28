macro_rules! deps {
    () => {
        Endian!();
        Note!();
        U32!();
    };
}

macro_rules! NoteHeader64 {
    () => {
        deps!();
        # [doc = " Note section entry header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct NoteHeader64 < E : Endian > { # [doc = " Length of the note's name."] # [doc = ""] # [doc = " Some known names are defined by the `ELF_NOTE_*` constants."] pub n_namesz : U32 < E > , # [doc = " Length of the note's descriptor."] # [doc = ""] # [doc = " The content of the descriptor depends on the note name and type."] pub n_descsz : U32 < E > , # [doc = " Type of the note."] # [doc = ""] # [doc = " One of the `NT_*` constants. The note name determines which"] # [doc = " `NT_*` constants are valid."] pub n_type : U32 < E > , }
    };
}

NoteHeader64!();