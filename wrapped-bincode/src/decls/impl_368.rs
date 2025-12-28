macro_rules! deps {
    () => {
        BorrowDecode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl < 'de , T , const N : usize , Context > BorrowDecode < 'de , Context > for [T ; N] where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { decoder . claim_bytes_read (core :: mem :: size_of :: < [T ; N] > ()) ? ; if unty :: type_equal :: < T , u8 > () { let mut buf = [0u8 ; N] ; decoder . reader () . read (& mut buf) ? ; let ptr = & mut buf as * mut _ as * mut [T ; N] ; let res = unsafe { ptr . read () } ; Ok (res) } else { let result = super :: impl_core :: collect_into_array (& mut (0 .. N) . map (| _ | { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; T :: borrow_decode (decoder) })) ; result . unwrap () } } }
    };
}

impl_368!();