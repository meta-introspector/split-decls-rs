macro_rules! deps {
    () => {
        BorrowDecoder!();
        DecodeError!();
        BorrowDecode!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Vec < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; if unty :: type_equal :: < T , u8 > () { decoder . claim_container_read :: < T > (len) ? ; let mut vec = alloc :: vec ! [0u8 ; len] ; decoder . reader () . read (& mut vec) ? ; Ok (unsafe { core :: mem :: transmute :: < Vec < u8 > , Vec < T > > (vec) }) } else { decoder . claim_container_read :: < T > (len) ? ; let mut vec = Vec :: with_capacity (len) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; vec . push (T :: borrow_decode (decoder) ?) ; } Ok (vec) } } }
    };
}

impl_53!();