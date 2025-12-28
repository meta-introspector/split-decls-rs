macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Bytes { pub fn megabytes (self) -> isize { self . 0 / 1024 / 1024 } }
    };
}

impl_15!();