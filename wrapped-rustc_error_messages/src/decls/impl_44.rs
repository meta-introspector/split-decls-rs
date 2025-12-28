macro_rules! deps {
    () => {
        SubdiagMessage!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl From < Cow < 'static , str > > for SubdiagMessage { fn from (s : Cow < 'static , str >) -> Self { SubdiagMessage :: Str (s) } }
    };
}

impl_44!();