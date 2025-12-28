macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! to_raw_tuple {
    () => {
        deps!();
        fn to_raw_tuple (trailers : & MessageTrailers , index : usize) -> (* const c_char , * const c_char) { unsafe { let addr = trailers . raw . trailers . wrapping_add (index) ; ((* addr) . key , (* addr) . value) } }
    };
}

to_raw_tuple!()