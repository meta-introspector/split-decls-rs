macro_rules! deps {
    () => {
        U32!();
        Endian!();
        LcStr!();
    };
}

macro_rules! SubLibraryCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SubLibraryCommand < E : Endian > { # [doc = " LC_SUB_LIBRARY"] pub cmd : U32 < E > , # [doc = " includes sub_library string"] pub cmdsize : U32 < E > , # [doc = " the sub_library name"] pub sub_library : LcStr < E > , }
    };
}

SubLibraryCommand!()