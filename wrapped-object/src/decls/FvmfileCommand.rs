macro_rules! deps {
    () => {
        U32!();
        Endian!();
        LcStr!();
    };
}

macro_rules! FvmfileCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FvmfileCommand < E : Endian > { # [doc = " LC_FVMFILE"] pub cmd : U32 < E > , # [doc = " includes pathname string"] pub cmdsize : U32 < E > , # [doc = " files pathname"] pub name : LcStr < E > , # [doc = " files virtual address"] pub header_addr : U32 < E > , }
    };
}

FvmfileCommand!()