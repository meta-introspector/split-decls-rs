macro_rules! TPL_DESCRIPTION {
    () => {
        const TPL_DESCRIPTION : & [u8] = include_bytes ! ("assets/init/description") ;
    };
}

TPL_DESCRIPTION!()