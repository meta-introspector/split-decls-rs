macro_rules! TZ_NAME_LEN {
    () => {
        # [doc = " The database reserves 40 bytes for each id."] const TZ_NAME_LEN : usize = 40 ;
    };
}

TZ_NAME_LEN!();