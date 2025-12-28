macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! OwnedKey {
    () => {
        deps!();
        struct OwnedKey (HKEY) ;
    };
}

OwnedKey!();