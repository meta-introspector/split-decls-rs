macro_rules! Buf {
    () => {
        struct Buf { bytes : [MaybeUninit < u8 > ; 40] , written : usize , }
    };
}

Buf!();