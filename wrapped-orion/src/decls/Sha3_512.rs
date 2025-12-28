macro_rules! deps {
    () => {
        Sha3!();
    };
}

macro_rules! Sha3_512 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA3-512 streaming state."] pub struct Sha3_512 { pub (crate) _state : Sha3 < SHA3_512_RATE > , }
    };
}

Sha3_512!()