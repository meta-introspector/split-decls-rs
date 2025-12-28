macro_rules! deps {
    () => {
        Endian!();
        U32!();
        U64Bytes!();
        U16!();
    };
}

macro_rules! Nlist64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Nlist64 < E : Endian > { # [doc = " index into the string table"] pub n_strx : U32 < E > , # [doc = " type flag, see below"] pub n_type : u8 , # [doc = " section number or NO_SECT"] pub n_sect : u8 , # [doc = " see <mach-o/stab.h>"] pub n_desc : U16 < E > , # [doc = " value of this symbol (or stab offset)"] pub n_value : U64Bytes < E > , }
    };
}

Nlist64!();