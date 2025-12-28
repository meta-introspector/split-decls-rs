macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        unsafe impl Send for Row { }
    };
}

impl_463!()