macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        unsafe impl Sync for Row { }
    };
}

impl_464!();