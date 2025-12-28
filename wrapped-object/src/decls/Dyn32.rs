macro_rules! deps {
    () => {
        Endian!();
        U32!();
        Dynamic!();
    };
}

macro_rules! Dyn32 {
    () => {
        deps!();
        # [doc = " Dynamic section entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Dyn32 < E : Endian > { # [doc = " Dynamic entry type."] pub d_tag : U32 < E > , # [doc = " Value (integer or address)."] pub d_val : U32 < E > , }
    };
}

Dyn32!();