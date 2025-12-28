macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser + Reset > Reset for SimpleHmacReset < D > { fn reset (& mut self) { Reset :: reset (& mut self . digest) ; self . digest . update (& self . ipad_key) ; } }
    };
}

impl_43!()