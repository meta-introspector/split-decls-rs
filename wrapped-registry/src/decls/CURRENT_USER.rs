macro_rules! CURRENT_USER {
    () => {
        # [doc = " The predefined `HKEY_CURRENT_USER` registry key."] pub const CURRENT_USER : & Key = & Key (HKEY_CURRENT_USER) ;
    };
}

CURRENT_USER!()