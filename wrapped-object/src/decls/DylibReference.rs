macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! DylibReference {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylibReference < E : Endian > { pub bitfield : U32 < E > , }
    };
}

DylibReference!()