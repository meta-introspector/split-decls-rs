macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! to_bytes_tuple {
    () => {
        deps!();
        fn to_bytes_tuple (trailers : & MessageTrailers , index : usize) -> (& [u8] , & [u8]) { unsafe { let (rkey , rvalue) = to_raw_tuple (& trailers , index) ; let key = CStr :: from_ptr (rkey) . to_bytes () ; let value = CStr :: from_ptr (rvalue) . to_bytes () ; (key , value) } }
    };
}

to_bytes_tuple!();