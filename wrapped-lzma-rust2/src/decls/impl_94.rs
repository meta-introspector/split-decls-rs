macro_rules! deps {
    () => {
        RangeReader!();
        RangeDecoder!();
        Result!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < R : RangeReader > RangeDecoder < R > { pub (crate) fn new_stream (mut inner : R) -> crate :: Result < Self > { let b = inner . try_read_u8 () ? ; if b != 0x00 { return Err (error_invalid_input ("range decoder first byte is not zero")) ; } let code = inner . read_u32_be () ? ; Ok (Self { inner , code , range : 0xFFFFFFFFu32 , }) } pub (crate) fn is_stream_finished (& self) -> bool { self . code == 0 } }
    };
}

impl_94!();