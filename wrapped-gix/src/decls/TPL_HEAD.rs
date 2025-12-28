macro_rules! TPL_HEAD {
    () => {
        const TPL_HEAD : & [u8] = include_bytes ! ("assets/init/HEAD") ;
    };
}

TPL_HEAD!();