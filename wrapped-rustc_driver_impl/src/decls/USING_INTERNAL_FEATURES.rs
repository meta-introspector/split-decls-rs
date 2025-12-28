macro_rules! USING_INTERNAL_FEATURES {
    () => {
        pub static USING_INTERNAL_FEATURES : AtomicBool = AtomicBool :: new (false) ;
    };
}

USING_INTERNAL_FEATURES!()