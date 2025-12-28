macro_rules! deps {
    () => {
        SysStack!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        unsafe impl Send for SysStack { }
    };
}

impl_66!()