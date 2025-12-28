macro_rules! deps {
    () => {
        Endian!();
        U32!();
        U64!();
    };
}

macro_rules! EntryPointCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct EntryPointCommand < E : Endian > { # [doc = " LC_MAIN only used in MH_EXECUTE filetypes"] pub cmd : U32 < E > , # [doc = " 24"] pub cmdsize : U32 < E > , # [doc = " file (__TEXT) offset of main()"] pub entryoff : U64 < E > , # [doc = " if not zero, initial stack size"] pub stacksize : U64 < E > , }
    };
}

EntryPointCommand!()