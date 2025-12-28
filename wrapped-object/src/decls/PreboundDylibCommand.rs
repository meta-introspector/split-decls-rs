macro_rules! deps {
    () => {
        Endian!();
        U32!();
        LcStr!();
    };
}

macro_rules! PreboundDylibCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct PreboundDylibCommand < E : Endian > { # [doc = " LC_PREBOUND_DYLIB"] pub cmd : U32 < E > , # [doc = " includes strings"] pub cmdsize : U32 < E > , # [doc = " library's path name"] pub name : LcStr < E > , # [doc = " number of modules in library"] pub nmodules : U32 < E > , # [doc = " bit vector of linked modules"] pub linked_modules : LcStr < E > , }
    };
}

PreboundDylibCommand!();