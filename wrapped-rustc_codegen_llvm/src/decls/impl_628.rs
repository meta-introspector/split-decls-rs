macro_rules! deps {
    () => {
        ModuleLlvm!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        unsafe impl Sync for ModuleLlvm { }
    };
}

impl_628!();