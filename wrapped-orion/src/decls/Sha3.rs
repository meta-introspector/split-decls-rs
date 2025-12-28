macro_rules! Sha3 {
    () => {
        # [derive (Clone)] # [doc = " SHA3 streaming state."] pub (crate) struct Sha3 < const RATE : usize > { pub (crate) state : [u64 ; 25] , pub (crate) buffer : [u8 ; RATE] , pub (crate) capacity : usize , leftover : usize , is_finalized : bool , }
    };
}

Sha3!()