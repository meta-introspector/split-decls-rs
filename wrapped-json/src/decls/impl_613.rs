macro_rules! deps {
    () => {
        RawValue!();
    };
}

macro_rules! impl_613 {
    () => {
        deps!();
        impl From < Box < RawValue > > for Box < str > { fn from (raw_value : Box < RawValue >) -> Self { RawValue :: into_owned (raw_value) } }
    };
}

impl_613!();