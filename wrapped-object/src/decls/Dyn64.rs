macro_rules! deps {
    () => {
        Endian!();
        U64!();
        Dynamic!();
    };
}

macro_rules! Dyn64 {
    () => {
        deps!();
        # [doc = " Dynamic section entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Dyn64 < E : Endian > { # [doc = " Dynamic entry type."] pub d_tag : U64 < E > , # [doc = " Value (integer or address)."] pub d_val : U64 < E > , }
    };
}

Dyn64!();