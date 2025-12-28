macro_rules! deps {
    () => {
        SimpleHmac!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > FixedOutput for SimpleHmac < D > { fn finalize_into (self , out : & mut Output < Self >) { let mut h = D :: new () ; h . update (& self . opad_key) ; h . update (self . digest . finalize ()) ; h . finalize_into (out) ; } }
    };
}

impl_32!();