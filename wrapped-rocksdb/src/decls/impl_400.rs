macro_rules! deps {
    () => {
        SstFileWriter!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        unsafe impl Sync for SstFileWriter < '_ > { }
    };
}

impl_400!();