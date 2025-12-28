macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > Update for SimpleHmacReset < D > { # [inline (always)] fn update (& mut self , data : & [u8]) { self . digest . update (data) ; } }
    };
}

impl_39!();