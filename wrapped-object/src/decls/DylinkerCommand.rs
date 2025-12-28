macro_rules! deps {
    () => {
        U32!();
        LcStr!();
        Endian!();
    };
}

macro_rules! DylinkerCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylinkerCommand < E : Endian > { # [doc = " LC_ID_DYLINKER, LC_LOAD_DYLINKER or LC_DYLD_ENVIRONMENT"] pub cmd : U32 < E > , # [doc = " includes pathname string"] pub cmdsize : U32 < E > , # [doc = " dynamic linker's path name"] pub name : LcStr < E > , }
    };
}

DylinkerCommand!();