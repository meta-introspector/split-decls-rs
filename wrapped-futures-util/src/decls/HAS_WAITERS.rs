macro_rules! HAS_WAITERS {
    () => {
        const HAS_WAITERS : usize = 1 << 1 ;
    };
}

HAS_WAITERS!();