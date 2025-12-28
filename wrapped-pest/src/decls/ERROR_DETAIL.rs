macro_rules! ERROR_DETAIL {
    () => {
        static ERROR_DETAIL : AtomicBool = AtomicBool :: new (false) ;
    };
}

ERROR_DETAIL!()