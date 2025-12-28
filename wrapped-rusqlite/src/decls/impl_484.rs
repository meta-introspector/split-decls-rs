macro_rules! deps {
    () => {
        Blob!();
        Value!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        # [cfg (feature = "uuid")] impl From < uuid :: Uuid > for Value { # [inline] fn from (id : uuid :: Uuid) -> Self { Self :: Blob (id . as_bytes () . to_vec ()) } }
    };
}

impl_484!();