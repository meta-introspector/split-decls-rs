macro_rules! deps {
    () => {
        SourceName!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl SourceName { # [inline] fn starts_with (byte : u8) -> bool { byte == b'0' || (b'0' <= byte && byte <= b'9') } }
    };
}

impl_120!()