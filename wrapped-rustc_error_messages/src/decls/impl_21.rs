macro_rules! deps {
    () => {
        DiagMessage!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < Cow < 'static , str > > for DiagMessage { fn from (s : Cow < 'static , str >) -> Self { DiagMessage :: Str (s) } }
    };
}

impl_21!()