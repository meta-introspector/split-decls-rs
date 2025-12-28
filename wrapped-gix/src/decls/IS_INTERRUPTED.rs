macro_rules! IS_INTERRUPTED {
    () => {
        # [doc = " The flag behind all utility functions in this module."] pub static IS_INTERRUPTED : AtomicBool = AtomicBool :: new (false) ;
    };
}

IS_INTERRUPTED!()