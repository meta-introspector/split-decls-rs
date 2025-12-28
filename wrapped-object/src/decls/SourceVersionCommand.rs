macro_rules! deps {
    () => {
        U64!();
        Endian!();
        U32!();
    };
}

macro_rules! SourceVersionCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SourceVersionCommand < E : Endian > { # [doc = " LC_SOURCE_VERSION"] pub cmd : U32 < E > , # [doc = " 16"] pub cmdsize : U32 < E > , # [doc = " A.B.C.D.E packed as a24.b10.c10.d10.e10"] pub version : U64 < E > , }
    };
}

SourceVersionCommand!()