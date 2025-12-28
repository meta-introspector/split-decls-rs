macro_rules! deps {
    () => {
        ModuleLlvm!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl Sync for ModuleLlvm { }
    };
}

impl_34!()