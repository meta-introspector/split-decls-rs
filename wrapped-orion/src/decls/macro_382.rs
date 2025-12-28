macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! macro_382 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl_serde_traits ! (PublicKey , to_bytes) ;
    };
}

macro_382!();