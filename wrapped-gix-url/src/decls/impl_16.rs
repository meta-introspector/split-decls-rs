macro_rules! deps {
    () => {
        Url!();
        Error!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl TryFrom < PathBuf > for Url { type Error = parse :: Error ; fn try_from (value : PathBuf) -> Result < Self , Self :: Error > { gix_path :: into_bstr (value) . try_into () } }
    };
}

impl_16!()