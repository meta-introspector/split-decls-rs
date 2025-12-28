macro_rules! deps {
    () => {
        ItemContainerId!();
        ModuleId!();
    };
}

macro_rules! macro_117 {
    () => {
        deps!();
        impl_from ! (ModuleId for ItemContainerId) ;
    };
}

macro_117!()