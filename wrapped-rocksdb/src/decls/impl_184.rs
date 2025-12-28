macro_rules! deps {
    () => {
        WriteBufferManagerWrapper!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        unsafe impl Sync for WriteBufferManagerWrapper { }
    };
}

impl_184!();