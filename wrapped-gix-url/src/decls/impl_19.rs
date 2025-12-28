macro_rules! deps {
    () => {
        Url!();
        Error!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Url { type Error = parse :: Error ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Self :: from_bytes (value) } }
    };
}

impl_19!();