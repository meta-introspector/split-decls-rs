macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl From < [u8 ; SIZE_OF_SHA1_DIGEST] > for ObjectId { fn from (v : [u8 ; 20]) -> Self { Self :: new_sha1 (v) } }
    };
}

impl_15!();