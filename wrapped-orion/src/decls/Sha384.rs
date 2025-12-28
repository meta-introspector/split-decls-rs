macro_rules! deps {
    () => {
        V384!();
    };
}

macro_rules! Sha384 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHA384 streaming state."] pub struct Sha384 { pub (crate) _state : State < WordU64 , V384 , SHA384_BLOCKSIZE , SHA384_OUTSIZE , N_CONSTS > , }
    };
}

Sha384!();