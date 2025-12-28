macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! Interface {
    () => {
        deps!();
        pub trait Interface { fn uuidof () -> GUID ; }
    };
}

Interface!();