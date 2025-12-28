macro_rules! deps {
    () => {
        DiagMessage!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl From < & 'static str > for DiagMessage { fn from (s : & 'static str) -> Self { DiagMessage :: Str (Cow :: Borrowed (s)) } }
    };
}

impl_20!()