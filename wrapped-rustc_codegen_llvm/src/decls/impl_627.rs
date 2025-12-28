macro_rules! deps {
    () => {
        ModuleLlvm!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        unsafe impl Send for ModuleLlvm { }
    };
}

impl_627!()