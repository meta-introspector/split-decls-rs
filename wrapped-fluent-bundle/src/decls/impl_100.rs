macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl From < String > for FluentValue < '_ > { fn from (s : String) -> Self { FluentValue :: String (s . into ()) } }
    };
}

impl_100!()