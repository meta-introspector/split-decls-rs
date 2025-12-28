macro_rules! CURRENT_CONFIG {
    () => {
        # [doc = " The predefined `HKEY_CURRENT_CONFIG` registry key."] pub const CURRENT_CONFIG : & Key = & Key (HKEY_CURRENT_CONFIG) ;
    };
}

CURRENT_CONFIG!()