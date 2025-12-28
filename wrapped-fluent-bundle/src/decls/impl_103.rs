macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'source > From < Cow < 'source , str > > for FluentValue < 'source > { fn from (s : Cow < 'source , str >) -> Self { FluentValue :: String (s) } }
    };
}

impl_103!();