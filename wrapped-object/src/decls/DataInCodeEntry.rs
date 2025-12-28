macro_rules! deps {
    () => {
        Endian!();
        U16!();
        U32!();
    };
}

macro_rules! DataInCodeEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DataInCodeEntry < E : Endian > { # [doc = " from mach_header to start of data range"] pub offset : U32 < E > , # [doc = " number of bytes in data range"] pub length : U16 < E > , # [doc = " a DICE_KIND_* value"] pub kind : U16 < E > , }
    };
}

DataInCodeEntry!();