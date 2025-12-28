macro_rules! deps {
    () => {
        WriteOptions!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        unsafe impl Send for WriteOptions { }
    };
}

impl_164!()