macro_rules! deps {
    () => {
        Dylib!();
        Endian!();
        U32!();
    };
}

macro_rules! DylibCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylibCommand < E : Endian > { # [doc = " LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB"] pub cmd : U32 < E > , # [doc = " includes pathname string"] pub cmdsize : U32 < E > , # [doc = " the library identification"] pub dylib : Dylib < E > , }
    };
}

DylibCommand!()