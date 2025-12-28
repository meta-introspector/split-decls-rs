macro_rules! deps {
    () => {
        U64!();
        Endian!();
    };
}

macro_rules! Relr64 {
    () => {
        deps!();
        # [doc = " 64-bit relative relocation table entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Relr64 < E : Endian > (pub U64 < E >) ;
    };
}

Relr64!();