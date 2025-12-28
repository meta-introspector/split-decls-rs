macro_rules! SINGLE_QUOTE {
    () => {
        pub const SINGLE_QUOTE : & 'static [(char , char)] = & [('\'' , '\'')] ;
    };
}

SINGLE_QUOTE!();