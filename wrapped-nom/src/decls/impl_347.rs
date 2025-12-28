macro_rules! deps {
    () => {
        Compare!();
        CompareResult!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < 'a , 'b , const N : usize > Compare < & 'b [u8 ; N] > for & 'a [u8] { # [inline (always)] fn compare (& self , t : & 'b [u8 ; N]) -> CompareResult { self . compare (& t [..]) } # [inline (always)] fn compare_no_case (& self , t : & 'b [u8 ; N]) -> CompareResult { self . compare_no_case (& t [..]) } }
    };
}

impl_347!();