macro_rules! deps {
    () => {
        Error!();
        Url!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl TryFrom < & str > for Url { type Error = parse :: Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Self :: from_bytes (value . into ()) } }
    };
}

impl_14!()