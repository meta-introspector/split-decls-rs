macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > MacMarker for SimpleHmacReset < D > { }
    };
}

impl_37!()