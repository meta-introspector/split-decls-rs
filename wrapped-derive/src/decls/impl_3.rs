macro_rules! deps {
    () => {
        CacheControl!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl CacheControl { pub fn is_public (& self) -> bool { ! self . private && self . public } }
    };
}

impl_3!()