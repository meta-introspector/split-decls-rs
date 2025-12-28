macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl Eq for BytesMut { }
    };
}

impl_196!()