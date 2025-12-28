macro_rules! deps {
    () => {
        Endian!();
        LcStr!();
        U32!();
    };
}

macro_rules! SubClientCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SubClientCommand < E : Endian > { # [doc = " LC_SUB_CLIENT"] pub cmd : U32 < E > , # [doc = " includes client string"] pub cmdsize : U32 < E > , # [doc = " the client name"] pub client : LcStr < E > , }
    };
}

SubClientCommand!()