macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! macro_381 {
    () => {
        deps!();
        impl_try_from_trait ! (PublicKey) ;
    };
}

macro_381!();