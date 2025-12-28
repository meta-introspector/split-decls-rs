macro_rules! deps {
    () => {
        Nonce!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Nonce { # [inline] pub fn new () -> Nonce { Nonce (NEXT_NONCE . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst)) } }
    };
}

impl_21!()