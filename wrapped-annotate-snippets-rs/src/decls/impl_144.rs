macro_rules! deps {
    () => {
        OptionCow!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'a , T : Into < Cow < 'a , str > > > From < Option < T > > for OptionCow < 'a > { fn from (value : Option < T >) -> Self { Self (value . map (Into :: into)) } }
    };
}

impl_144!();