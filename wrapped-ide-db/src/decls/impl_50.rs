macro_rules! deps {
    () => {
        Documentation!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Documentation { pub fn new (s : String) -> Self { Documentation (s) } pub fn as_str (& self) -> & str { & self . 0 } }
    };
}

impl_50!();