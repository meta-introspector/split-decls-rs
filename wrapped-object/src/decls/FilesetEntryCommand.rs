macro_rules! deps {
    () => {
        U64!();
        Endian!();
        LcStr!();
        U32!();
    };
}

macro_rules! FilesetEntryCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FilesetEntryCommand < E : Endian > { pub cmd : U32 < E > , # [doc = " includes id string"] pub cmdsize : U32 < E > , # [doc = " memory address of the dylib"] pub vmaddr : U64 < E > , # [doc = " file offset of the dylib"] pub fileoff : U64 < E > , # [doc = " contained entry id"] pub entry_id : LcStr < E > , # [doc = " entry_id is 32-bits long, so this is the reserved padding"] pub reserved : U32 < E > , }
    };
}

FilesetEntryCommand!();