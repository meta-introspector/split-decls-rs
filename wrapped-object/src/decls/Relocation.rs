macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! Relocation {
    () => {
        deps!();
        # [doc = " A relocation entry."] # [doc = ""] # [doc = " Mach-O relocations have plain and scattered variants, with the"] # [doc = " meaning of the fields depending on the variant."] # [doc = ""] # [doc = " This type provides functions for determining whether the relocation"] # [doc = " is scattered, and for accessing the fields of each variant."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Relocation < E : Endian > { pub r_word0 : U32 < E > , pub r_word1 : U32 < E > , }
    };
}

Relocation!()