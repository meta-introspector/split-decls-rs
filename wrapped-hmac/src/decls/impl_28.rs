macro_rules! deps {
    () => {
        SimpleHmac!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > MacMarker for SimpleHmac < D > { }
    };
}

impl_28!();