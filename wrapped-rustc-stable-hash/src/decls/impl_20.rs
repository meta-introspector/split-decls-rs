macro_rules! deps {
    () => {
        SipHasher128!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Default for SipHasher128 { fn default () -> SipHasher128 { SipHasher128 :: new_with_keys (0 , 0) } }
    };
}

impl_20!()