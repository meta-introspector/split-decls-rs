macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! Relr32 {
    () => {
        deps!();
        # [doc = " 32-bit relative relocation table entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Relr32 < E : Endian > (pub U32 < E >) ;
    };
}

Relr32!();