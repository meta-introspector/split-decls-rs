macro_rules! deps {
    () => {
        SubdiagMessage!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl From < & 'static str > for SubdiagMessage { fn from (s : & 'static str) -> Self { SubdiagMessage :: Str (Cow :: Borrowed (s)) } }
    };
}

impl_43!()