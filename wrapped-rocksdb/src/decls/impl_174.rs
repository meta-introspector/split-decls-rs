macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        unsafe impl Sync for Options { }
    };
}

impl_174!();