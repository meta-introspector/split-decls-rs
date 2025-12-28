macro_rules! deps {
    () => {
        InstanceState!();
    };
}

macro_rules! eLocal {
    () => {
        deps!();
        pub const eLocal : InstanceState = 1 ;
    };
}

eLocal!();