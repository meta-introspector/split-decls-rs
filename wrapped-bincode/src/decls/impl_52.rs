macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Vec < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; if unty :: type_equal :: < T , u8 > () { decoder . claim_container_read :: < T > (len) ? ; let mut vec = alloc :: vec ! [0u8 ; len] ; decoder . reader () . read (& mut vec) ? ; Ok (unsafe { core :: mem :: transmute :: < Vec < u8 > , Vec < T > > (vec) }) } else { decoder . claim_container_read :: < T > (len) ? ; let mut vec = Vec :: with_capacity (len) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; vec . push (T :: decode (decoder) ?) ; } Ok (vec) } } }
    };
}

impl_52!()