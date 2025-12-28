macro_rules! deps {
    () => {
        Blob!();
        Value!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        # [cfg (feature = "i128_blob")] impl From < i128 > for Value { # [inline] fn from (i : i128) -> Self { Self :: Blob (i128 :: to_be_bytes (i ^ (1_i128 << 127)) . to_vec ()) } }
    };
}

impl_483!()