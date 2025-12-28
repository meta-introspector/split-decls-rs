macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        unsafe impl Send for Row < '_ > { }
    };
}

impl_67!()