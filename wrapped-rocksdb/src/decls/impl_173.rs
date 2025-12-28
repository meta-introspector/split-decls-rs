macro_rules! deps {
    () => {
        WriteBufferManagerWrapper!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        unsafe impl Send for WriteBufferManagerWrapper { }
    };
}

impl_173!();