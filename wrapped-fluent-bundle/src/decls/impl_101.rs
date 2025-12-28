macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'source > From < & 'source String > for FluentValue < 'source > { fn from (s : & 'source String) -> Self { FluentValue :: String (s . into ()) } }
    };
}

impl_101!()