macro_rules! deps {
    () => {
        OptionCow!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'a > From < & 'a Cow < 'a , str > > for OptionCow < 'a > { fn from (value : & 'a Cow < 'a , str >) -> Self { Self (Some (Cow :: Borrowed (value))) } }
    };
}

impl_145!()