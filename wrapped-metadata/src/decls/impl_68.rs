macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        unsafe impl Sync for Row < '_ > { }
    };
}

impl_68!();