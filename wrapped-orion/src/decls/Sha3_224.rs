macro_rules! deps {
    () => {
        Sha3!();
    };
}

macro_rules! Sha3_224 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA3-224 streaming state."] pub struct Sha3_224 { pub (crate) _state : Sha3 < SHA3_224_RATE > , }
    };
}

Sha3_224!()