macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! LinkerOptionCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct LinkerOptionCommand < E : Endian > { # [doc = " LC_LINKER_OPTION only used in MH_OBJECT filetypes"] pub cmd : U32 < E > , pub cmdsize : U32 < E > , # [doc = " number of strings"] pub count : U32 < E > , }
    };
}

LinkerOptionCommand!();