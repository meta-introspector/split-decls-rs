macro_rules! deps {
    () => {
        OptionCow!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for OptionCow < 'a > { fn from (value : & 'a str) -> Self { Self (Some (Cow :: Borrowed (value))) } }
    };
}

impl_147!();