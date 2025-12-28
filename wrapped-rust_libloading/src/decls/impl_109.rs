macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        unsafe impl Send for Library { }
    };
}

impl_109!()