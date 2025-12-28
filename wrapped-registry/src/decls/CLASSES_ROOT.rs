macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! CLASSES_ROOT {
    () => {
        deps!();
        # [doc = " The predefined `HKEY_CLASSES_ROOT` registry key."] pub const CLASSES_ROOT : & Key = & Key (HKEY_CLASSES_ROOT) ;
    };
}

CLASSES_ROOT!();