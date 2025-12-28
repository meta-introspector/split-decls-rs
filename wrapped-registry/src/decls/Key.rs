macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! Key {
    () => {
        deps!();
        # [doc = " A registry key."] # [repr (transparent)] # [derive (Debug)] pub struct Key (pub (crate) HKEY) ;
    };
}

Key!()