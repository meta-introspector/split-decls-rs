macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! TwolevelHint {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct TwolevelHint < E : Endian > { pub bitfield : U32 < E > , }
    };
}

TwolevelHint!();