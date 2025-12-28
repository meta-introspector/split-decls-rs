macro_rules! deps {
    () => {
        ModuleId!();
        ItemContainerId!();
    };
}

macro_rules! macro_689 {
    () => {
        deps!();
        impl_from ! (ModuleId for ItemContainerId) ;
    };
}

macro_689!();