macro_rules! ULONG {
    () => {
        pub type ULONG = raw :: c_ulong ;
    };
}

ULONG!()