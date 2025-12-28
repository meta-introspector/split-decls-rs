macro_rules! deps {
    () => {
        Compare!();
        CompareResult!();
        AsBytes!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < 'a , 'b > Compare < & 'b [u8] > for & 'a str { # [inline (always)] fn compare (& self , t : & 'b [u8]) -> CompareResult { AsBytes :: as_bytes (self) . compare (t) } # [inline (always)] fn compare_no_case (& self , t : & 'b [u8]) -> CompareResult { AsBytes :: as_bytes (self) . compare_no_case (t) } }
    };
}

impl_329!();