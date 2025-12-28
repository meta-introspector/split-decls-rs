macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! ThreadCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ThreadCommand < E : Endian > { # [doc = " LC_THREAD or  LC_UNIXTHREAD"] pub cmd : U32 < E > , # [doc = " total size of this command"] pub cmdsize : U32 < E > , }
    };
}

ThreadCommand!()