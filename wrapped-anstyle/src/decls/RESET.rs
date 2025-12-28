macro_rules! RESET {
    () => {
        pub (crate) const RESET : & str = "\x1B[0m" ;
    };
}

RESET!();