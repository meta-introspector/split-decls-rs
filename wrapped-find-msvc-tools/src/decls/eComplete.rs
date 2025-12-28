macro_rules! deps {
    () => {
        InstanceState!();
    };
}

macro_rules! eComplete {
    () => {
        deps!();
        pub const eComplete : InstanceState = - 1i32 as u32 ;
    };
}

eComplete!()