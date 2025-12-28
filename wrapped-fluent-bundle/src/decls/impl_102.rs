macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'source > From < & 'source str > for FluentValue < 'source > { fn from (s : & 'source str) -> Self { FluentValue :: String (s . into ()) } }
    };
}

impl_102!();