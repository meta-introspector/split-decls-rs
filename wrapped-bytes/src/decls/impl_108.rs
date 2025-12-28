macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl From < & 'static str > for Bytes { fn from (slice : & 'static str) -> Bytes { Bytes :: from_static (slice . as_bytes ()) } }
    };
}

impl_108!();