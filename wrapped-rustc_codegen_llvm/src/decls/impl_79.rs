macro_rules! deps {
    () => {
        ModuleBuffer!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        unsafe impl Send for ModuleBuffer { }
    };
}

impl_79!();