macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a str > for & 'a RelativePath { type Error = Error ; fn try_from (value : & 'a str) -> Result < Self , Self :: Error > { relative_path_from_value_and_path (value . into () , Path :: new (value)) } }
    };
}

impl_76!();