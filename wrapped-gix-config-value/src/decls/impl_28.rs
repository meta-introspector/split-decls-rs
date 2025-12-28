macro_rules! deps {
    () => {
        Integer!();
        Suffix!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Integer { # [doc = " Canonicalize values as simple decimal numbers."] # [doc = " An optional suffix of k, m, or g (case-insensitive), will cause the"] # [doc = " value to be multiplied by 1024 (k), 1048576 (m), or 1073741824 (g) respectively."] # [doc = ""] # [doc = " Returns the result if there is no multiplication overflow."] pub fn to_decimal (& self) -> Option < i64 > { match self . suffix { None => Some (self . value) , Some (suffix) => match suffix { Suffix :: Kibi => self . value . checked_mul (1024) , Suffix :: Mebi => self . value . checked_mul (1024 * 1024) , Suffix :: Gibi => self . value . checked_mul (1024 * 1024 * 1024) , } , } } }
    };
}

impl_28!()