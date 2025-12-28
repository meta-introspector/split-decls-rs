macro_rules! sc_reduce32 {
    () => {
        pub fn sc_reduce32 (s : & mut [u8 ; 32]) { let mut t = [0u8 ; 64] ; t [0 .. 32] . copy_from_slice (s) ; sc_reduce (& mut t) ; s . copy_from_slice (& t [0 .. 32]) ; }
    };
}

sc_reduce32!()