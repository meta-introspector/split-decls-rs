macro_rules! deps {
    () => {
        TokenStaticData!();
    };
}

macro_rules! TokenStore {
    () => {
        deps!();
        # [derive (Default , Debug)] pub struct TokenStore (Vec < TokenStaticData >) ;
    };
}

TokenStore!();