macro_rules! deps {
    () => {
        BlockBasedOptions!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        unsafe impl Sync for BlockBasedOptions { }
    };
}

impl_178!();