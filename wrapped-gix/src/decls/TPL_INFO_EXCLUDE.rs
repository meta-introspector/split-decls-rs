macro_rules! TPL_INFO_EXCLUDE {
    () => {
        const TPL_INFO_EXCLUDE : & [u8] = include_bytes ! ("assets/init/info/exclude") ;
    };
}

TPL_INFO_EXCLUDE!()