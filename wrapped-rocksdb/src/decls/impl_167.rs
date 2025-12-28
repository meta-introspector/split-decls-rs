macro_rules! deps {
    () => {
        BlockBasedOptions!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        unsafe impl Send for BlockBasedOptions { }
    };
}

impl_167!()