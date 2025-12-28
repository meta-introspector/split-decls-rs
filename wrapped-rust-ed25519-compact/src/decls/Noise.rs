macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! Noise {
    () => {
        deps!();
        # [doc = " Noise, for non-deterministic signatures."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct Noise ([u8 ; Noise :: BYTES]) ;
    };
}

Noise!();