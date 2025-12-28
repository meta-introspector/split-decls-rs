macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! TwolevelHint {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct TwolevelHint < E : Endian > { pub bitfield : U32 < E > , }
    };
}

TwolevelHint!()