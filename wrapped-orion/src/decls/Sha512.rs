macro_rules! deps {
    () => {
        V512!();
    };
}

macro_rules! Sha512 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA512 streaming state."] pub struct Sha512 { pub (crate) _state : State < WordU64 , V512 , SHA512_BLOCKSIZE , SHA512_OUTSIZE , N_CONSTS > , }
    };
}

Sha512!()