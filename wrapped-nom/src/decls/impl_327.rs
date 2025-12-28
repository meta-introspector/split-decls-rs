macro_rules! deps {
    () => {
        CompareResult!();
        Compare!();
        AsBytes!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'a , 'b > Compare < & 'b str > for & 'a [u8] { # [inline (always)] fn compare (& self , t : & 'b str) -> CompareResult { self . compare (AsBytes :: as_bytes (t)) } # [inline (always)] fn compare_no_case (& self , t : & 'b str) -> CompareResult { self . compare_no_case (AsBytes :: as_bytes (t)) } }
    };
}

impl_327!();