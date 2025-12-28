macro_rules! deps {
    () => {
        Endian!();
        U16!();
        Version!();
    };
}

macro_rules! Versym {
    () => {
        deps!();
        # [doc = " Version symbol information"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Versym < E : Endian > (pub U16 < E >) ;
    };
}

Versym!();