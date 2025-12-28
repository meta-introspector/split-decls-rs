macro_rules! deps {
    () => {
        V256!();
    };
}

macro_rules! Sha256 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA256 streaming state."] pub struct Sha256 { pub (crate) _state : State < WordU32 , V256 , SHA256_BLOCKSIZE , SHA256_OUTSIZE , N_CONSTS > , }
    };
}

Sha256!();