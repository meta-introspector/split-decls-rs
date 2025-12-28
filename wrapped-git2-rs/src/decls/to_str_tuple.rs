macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! to_str_tuple {
    () => {
        deps!();
        fn to_str_tuple (trailers : & MessageTrailers , index : usize) -> (& str , & str) { unsafe { let (rkey , rvalue) = to_raw_tuple (& trailers , index) ; let key = CStr :: from_ptr (rkey) . to_str () . unwrap () ; let value = CStr :: from_ptr (rvalue) . to_str () . unwrap () ; (key , value) } }
    };
}

to_str_tuple!();