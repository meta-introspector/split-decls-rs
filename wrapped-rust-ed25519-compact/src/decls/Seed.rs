macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! Seed {
    () => {
        deps!();
        # [doc = " A seed, which a key pair can be derived from."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct Seed ([u8 ; Seed :: BYTES]) ;
    };
}

Seed!();