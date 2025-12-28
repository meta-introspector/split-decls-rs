macro_rules! deps {
    () => {
        SstFileWriter!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        unsafe impl Send for SstFileWriter < '_ > { }
    };
}

impl_399!()