macro_rules! deps {
    () => {
        DiagMessage!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl From < String > for DiagMessage { fn from (s : String) -> Self { DiagMessage :: Str (Cow :: Owned (s)) } }
    };
}

impl_47!()