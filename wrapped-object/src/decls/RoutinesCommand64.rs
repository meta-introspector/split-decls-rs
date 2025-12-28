macro_rules! deps {
    () => {
        Endian!();
        U32!();
        U64!();
    };
}

macro_rules! RoutinesCommand64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct RoutinesCommand64 < E : Endian > { # [doc = " LC_ROUTINES_64"] pub cmd : U32 < E > , # [doc = " total size of this command"] pub cmdsize : U32 < E > , # [doc = " address of initialization routine"] pub init_address : U64 < E > , # [doc = " index into the module table that the init routine is defined in"] pub init_module : U64 < E > , pub reserved1 : U64 < E > , pub reserved2 : U64 < E > , pub reserved3 : U64 < E > , pub reserved4 : U64 < E > , pub reserved5 : U64 < E > , pub reserved6 : U64 < E > , }
    };
}

RoutinesCommand64!();