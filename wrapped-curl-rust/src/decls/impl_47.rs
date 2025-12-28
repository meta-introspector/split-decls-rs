macro_rules! deps {
    () => {
        EasyData!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        unsafe impl Send for EasyData { }
    };
}

impl_47!()