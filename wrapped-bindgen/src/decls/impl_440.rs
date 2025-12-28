macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        unsafe impl Sync for File { }
    };
}

impl_440!();