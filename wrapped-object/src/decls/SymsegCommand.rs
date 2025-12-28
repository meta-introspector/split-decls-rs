macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! SymsegCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SymsegCommand < E : Endian > { # [doc = " LC_SYMSEG"] pub cmd : U32 < E > , # [doc = " sizeof(struct SymsegCommand)"] pub cmdsize : U32 < E > , # [doc = " symbol segment offset"] pub offset : U32 < E > , # [doc = " symbol segment size in bytes"] pub size : U32 < E > , }
    };
}

SymsegCommand!()