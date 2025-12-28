macro_rules! deps {
    () => {
        InstanceState!();
    };
}

macro_rules! eNone {
    () => {
        deps!();
        pub const eNone : InstanceState = 0 ;
    };
}

eNone!();