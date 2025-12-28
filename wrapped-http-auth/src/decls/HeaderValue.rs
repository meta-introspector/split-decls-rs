macro_rules! deps {
    () => {
        ToStrError!();
    };
}

macro_rules! HeaderValue {
    () => {
        deps!();
        # [doc = " A trait for the parts needed from http crate 0.2 or 1.0's `HeaderValue` type."] # [cfg (any (feature = "http" , feature = "http10"))] pub trait HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > ; }
    };
}

HeaderValue!();