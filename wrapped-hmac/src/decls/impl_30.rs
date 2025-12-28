macro_rules! deps {
    () => {
        SimpleHmac!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > Update for SimpleHmac < D > { # [inline (always)] fn update (& mut self , data : & [u8]) { self . digest . update (data) ; } }
    };
}

impl_30!()