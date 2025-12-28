macro_rules! deps {
    () => {
        ModuleBuffer!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        unsafe impl Sync for ModuleBuffer { }
    };
}

impl_80!()