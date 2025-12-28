macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > FixedOutput for SimpleHmacReset < D > { fn finalize_into (self , out : & mut Output < Self >) { let mut h = D :: new () ; h . update (& self . opad_key) ; h . update (self . digest . finalize ()) ; h . finalize_into (out) ; } }
    };
}

impl_41!()