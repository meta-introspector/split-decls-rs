macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! CURRENT_CONFIG {
    () => {
        deps!();
        # [doc = " The predefined `HKEY_CURRENT_CONFIG` registry key."] pub const CURRENT_CONFIG : & Key = & Key (HKEY_CURRENT_CONFIG) ;
    };
}

CURRENT_CONFIG!()