macro_rules! deps {
    () => {
        ObjectId!();
        Prefix!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < ObjectId > for Prefix { fn from (oid : ObjectId) -> Self { Prefix { bytes : oid , hex_len : oid . kind () . len_in_hex () , } } }
    };
}

impl_29!();