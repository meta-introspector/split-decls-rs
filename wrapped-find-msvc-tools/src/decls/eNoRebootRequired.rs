macro_rules! deps {
    () => {
        InstanceState!();
    };
}

macro_rules! eNoRebootRequired {
    () => {
        deps!();
        pub const eNoRebootRequired : InstanceState = 4 ;
    };
}

eNoRebootRequired!();