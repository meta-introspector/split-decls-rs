macro_rules! deps {
    () => {
        InstanceState!();
    };
}

macro_rules! eRegistered {
    () => {
        deps!();
        pub const eRegistered : InstanceState = 2 ;
    };
}

eRegistered!()