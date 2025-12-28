macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! CURRENT_USER {
    () => {
        deps!();
        # [doc = " The predefined `HKEY_CURRENT_USER` registry key."] pub const CURRENT_USER : & Key = & Key (HKEY_CURRENT_USER) ;
    };
}

CURRENT_USER!()