macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        unsafe impl Send for BytesMut { }
    };
}

impl_216!()