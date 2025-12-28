macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        unsafe impl Sync for BytesMut { }
    };
}

impl_217!()