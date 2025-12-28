macro_rules! deps {
    () => {
        ModuleLlvm!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        unsafe impl Send for ModuleLlvm { }
    };
}

impl_33!()