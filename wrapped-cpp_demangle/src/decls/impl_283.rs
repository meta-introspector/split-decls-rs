macro_rules! deps {
    () => {
        IndexStr!();
        ClosureTypeName!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl ClosureTypeName { # [inline] fn starts_with (byte : u8 , input : & IndexStr) -> bool { byte == b'U' && input . peek_second () . map (| b | b == b'l') . unwrap_or (false) } }
    };
}

impl_283!();