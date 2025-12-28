macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! IdentCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct IdentCommand < E : Endian > { # [doc = " LC_IDENT"] pub cmd : U32 < E > , # [doc = " strings that follow this command"] pub cmdsize : U32 < E > , }
    };
}

IdentCommand!();