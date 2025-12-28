macro_rules! deps {
    () => {
        Version!();
        U32!();
        U16!();
        Endian!();
    };
}

macro_rules! Verdef {
    () => {
        deps!();
        # [doc = " Version definition sections"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Verdef < E : Endian > { # [doc = " Version revision"] pub vd_version : U16 < E > , # [doc = " Version information"] pub vd_flags : U16 < E > , # [doc = " Version Index"] pub vd_ndx : U16 < E > , # [doc = " Number of associated aux entries"] pub vd_cnt : U16 < E > , # [doc = " Version name hash value"] pub vd_hash : U32 < E > , # [doc = " Offset in bytes to verdaux array"] pub vd_aux : U32 < E > , # [doc = " Offset in bytes to next verdef entry"] pub vd_next : U32 < E > , }
    };
}

Verdef!()