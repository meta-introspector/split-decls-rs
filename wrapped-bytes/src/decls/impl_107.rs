macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl From < & 'static [u8] > for Bytes { fn from (slice : & 'static [u8]) -> Bytes { Bytes :: from_static (slice) } }
    };
}

impl_107!()