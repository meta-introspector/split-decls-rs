macro_rules! deps {
    () => {
        ULONGLONG!();
    };
}

macro_rules! PULONGLONG {
    () => {
        deps!();
        pub type PULONGLONG = * mut ULONGLONG ;
    };
}

PULONGLONG!();