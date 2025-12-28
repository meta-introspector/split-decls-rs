macro_rules! deps {
    () => {
        GetTimezoneError!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl From < std :: io :: Error > for GetTimezoneError { fn from (orig : std :: io :: Error) -> Self { GetTimezoneError :: IoError (orig) } }
    };
}

impl_5!()