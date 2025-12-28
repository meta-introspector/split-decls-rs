macro_rules! deps {
    () => {
        CompareResult!();
        Compare!();
        Error!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < 'a , 'b > Compare < & 'b [u8] > for & 'a [u8] { # [inline (always)] fn compare (& self , t : & 'b [u8]) -> CompareResult { let pos = self . iter () . zip (t . iter ()) . position (| (a , b) | a != b) ; match pos { Some (_) => CompareResult :: Error , None => { if self . len () >= t . len () { CompareResult :: Ok } else { CompareResult :: Incomplete } } } } # [inline (always)] fn compare_no_case (& self , t : & 'b [u8]) -> CompareResult { if self . iter () . zip (t) . any (| (a , b) | lowercase_byte (* a) != lowercase_byte (* b)) { CompareResult :: Error } else if self . len () < t . len () { CompareResult :: Incomplete } else { CompareResult :: Ok } } }
    };
}

impl_326!();