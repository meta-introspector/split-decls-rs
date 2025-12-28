macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl < Context , T , const N : usize > Decode < Context > for [T ; N] where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (core :: mem :: size_of :: < [T ; N] > ()) ? ; if unty :: type_equal :: < T , u8 > () { let mut buf = [0u8 ; N] ; decoder . reader () . read (& mut buf) ? ; let ptr = & mut buf as * mut _ as * mut [T ; N] ; let res = unsafe { ptr . read () } ; Ok (res) } else { let result = super :: impl_core :: collect_into_array (& mut (0 .. N) . map (| _ | { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; T :: decode (decoder) })) ; result . unwrap () } } }
    };
}

impl_367!();