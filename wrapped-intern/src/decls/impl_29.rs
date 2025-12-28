macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Interned < str > { # [inline] pub fn new_str (s : & str) -> Self { Self :: new_generic (s) } }
    };
}

impl_29!()