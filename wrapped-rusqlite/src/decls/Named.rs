macro_rules! deps {
    () => {
        SmallCString!();
    };
}

macro_rules! Named {
    () => {
        deps!();
        pub enum Named < 'a > { Small (SmallCString) , C (& 'a CStr) , }
    };
}

Named!();