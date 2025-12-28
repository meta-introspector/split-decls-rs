macro_rules! IS_LOCKED {
    () => {
        const IS_LOCKED : usize = 1 << 0 ;
    };
}

IS_LOCKED!();