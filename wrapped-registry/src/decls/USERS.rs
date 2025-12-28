macro_rules! USERS {
    () => {
        # [doc = " The predefined `HKEY_USERS` registry key."] pub const USERS : & Key = & Key (HKEY_USERS) ;
    };
}

USERS!()