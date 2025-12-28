macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! DylibReference {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylibReference < E : Endian > { pub bitfield : U32 < E > , }
    };
}

DylibReference!();