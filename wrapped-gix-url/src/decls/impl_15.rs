macro_rules! deps {
    () => {
        Url!();
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl TryFrom < String > for Url { type Error = parse :: Error ; fn try_from (value : String) -> Result < Self , Self :: Error > { Self :: from_bytes (value . as_str () . into ()) } }
    };
}

impl_15!()