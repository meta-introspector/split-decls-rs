macro_rules! deps {
    () => {
        Sha3!();
    };
}

macro_rules! Sha3_384 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA3-384 streaming state."] pub struct Sha3_384 { pub (crate) _state : Sha3 < SHA3_384_RATE > , }
    };
}

Sha3_384!();