macro_rules! deps {
    () => {
        U32x4!();
    };
}

macro_rules! ChaCha20 {
    () => {
        deps!();
        pub (crate) struct ChaCha20 { state : [U32x4 ; 4] , internal_counter : u32 , is_ietf : bool , }
    };
}

ChaCha20!();