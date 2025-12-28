macro_rules! deps {
    () => {
        SubdiagMessage!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl From < String > for SubdiagMessage { fn from (s : String) -> Self { SubdiagMessage :: Str (Cow :: Owned (s)) } }
    };
}

impl_14!()