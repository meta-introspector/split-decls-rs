macro_rules! deps {
    () => {
        U16!();
        Endian!();
        Version!();
        U32!();
    };
}

macro_rules! Verneed {
    () => {
        deps!();
        # [doc = " Version dependency."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Verneed < E : Endian > { # [doc = " Version of structure"] pub vn_version : U16 < E > , # [doc = " Number of associated aux entries"] pub vn_cnt : U16 < E > , # [doc = " Offset of filename for this dependency"] pub vn_file : U32 < E > , # [doc = " Offset in bytes to vernaux array"] pub vn_aux : U32 < E > , # [doc = " Offset in bytes to next verneed entry"] pub vn_next : U32 < E > , }
    };
}

Verneed!();