macro_rules! deps {
    () => {
        Sha3!();
    };
}

macro_rules! Sha3_256 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA3-256 streaming state."] pub struct Sha3_256 { pub (crate) _state : Sha3 < SHA3_256_RATE > , }
    };
}

Sha3_256!()