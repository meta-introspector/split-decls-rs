macro_rules! deps {
    () => {
        U32!();
        Fvmlib!();
        Endian!();
    };
}

macro_rules! FvmlibCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FvmlibCommand < E : Endian > { # [doc = " LC_IDFVMLIB or LC_LOADFVMLIB"] pub cmd : U32 < E > , # [doc = " includes pathname string"] pub cmdsize : U32 < E > , # [doc = " the library identification"] pub fvmlib : Fvmlib < E > , }
    };
}

FvmlibCommand!()