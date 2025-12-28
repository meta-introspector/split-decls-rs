macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        unsafe impl Send for Select < '_ > { }
    };
}

impl_177!()