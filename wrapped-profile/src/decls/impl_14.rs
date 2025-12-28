macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Bytes { pub fn new (bytes : isize) -> Bytes { Bytes (bytes) } }
    };
}

impl_14!()