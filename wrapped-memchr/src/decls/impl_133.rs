macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        unsafe impl < 'h > Send for Iter < 'h > { }
    };
}

impl_133!();