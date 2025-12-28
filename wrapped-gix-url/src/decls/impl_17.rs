macro_rules! deps {
    () => {
        Url!();
        Error!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl TryFrom < & Path > for Url { type Error = parse :: Error ; fn try_from (value : & Path) -> Result < Self , Self :: Error > { gix_path :: into_bstr (value) . try_into () } }
    };
}

impl_17!();