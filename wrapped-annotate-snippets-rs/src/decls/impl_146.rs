macro_rules! deps {
    () => {
        OptionCow!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a > From < Cow < 'a , str > > for OptionCow < 'a > { fn from (value : Cow < 'a , str >) -> Self { Self (Some (value)) } }
    };
}

impl_146!()