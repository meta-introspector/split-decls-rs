macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! LinkeditDataCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct LinkeditDataCommand < E : Endian > { # [doc = " `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,"] # [doc = " `LC_DATA_IN_CODE`, `LC_DYLIB_CODE_SIGN_DRS`, `LC_LINKER_OPTIMIZATION_HINT`,"] # [doc = " `LC_DYLD_EXPORTS_TRIE`, or `LC_DYLD_CHAINED_FIXUPS`."] pub cmd : U32 < E > , # [doc = " sizeof(struct LinkeditDataCommand)"] pub cmdsize : U32 < E > , # [doc = " file offset of data in __LINKEDIT segment"] pub dataoff : U32 < E > , # [doc = " file size of data in __LINKEDIT segment"] pub datasize : U32 < E > , }
    };
}

LinkeditDataCommand!();