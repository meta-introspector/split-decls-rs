macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! UuidCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct UuidCommand < E : Endian > { # [doc = " LC_UUID"] pub cmd : U32 < E > , # [doc = " sizeof(struct UuidCommand)"] pub cmdsize : U32 < E > , # [doc = " the 128-bit uuid"] pub uuid : [u8 ; 16] , }
    };
}

UuidCommand!()