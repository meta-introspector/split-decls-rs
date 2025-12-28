macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! USERS {
    () => {
        deps!();
        # [doc = " The predefined `HKEY_USERS` registry key."] pub const USERS : & Key = & Key (HKEY_USERS) ;
    };
}

USERS!();