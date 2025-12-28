macro_rules! deps {
    () => {
        CompareResult!();
        Compare!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < 'a , const N : usize > Compare < [u8 ; N] > for & 'a [u8] { # [inline (always)] fn compare (& self , t : [u8 ; N]) -> CompareResult { self . compare (& t [..]) } # [inline (always)] fn compare_no_case (& self , t : [u8 ; N]) -> CompareResult { self . compare_no_case (& t [..]) } }
    };
}

impl_346!();