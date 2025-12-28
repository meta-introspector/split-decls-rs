macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! PrebindCksumCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct PrebindCksumCommand < E : Endian > { # [doc = " LC_PREBIND_CKSUM"] pub cmd : U32 < E > , # [doc = " sizeof(struct PrebindCksumCommand)"] pub cmdsize : U32 < E > , # [doc = " the check sum or zero"] pub cksum : U32 < E > , }
    };
}

PrebindCksumCommand!();