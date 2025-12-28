macro_rules! deps {
    () => {
        Endian!();
        LcStr!();
        U32!();
    };
}

macro_rules! RpathCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct RpathCommand < E : Endian > { # [doc = " LC_RPATH"] pub cmd : U32 < E > , # [doc = " includes string"] pub cmdsize : U32 < E > , # [doc = " path to add to run path"] pub path : LcStr < E > , }
    };
}

RpathCommand!();