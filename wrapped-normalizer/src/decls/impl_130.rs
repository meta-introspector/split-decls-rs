macro_rules! deps {
    () => {
        IsNormalizedSinkUtf16!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        # [cfg (feature = "utf16_iter")] impl write16 :: Write16 for IsNormalizedSinkUtf16 < '_ > { fn write_slice (& mut self , s : & [u16]) -> core :: fmt :: Result { # [expect (clippy :: indexing_slicing)] if core :: ptr :: eq (s . as_ptr () , self . expect . as_ptr ()) { self . expect = & self . expect [s . len () ..] ; Ok (()) } else { Err (core :: fmt :: Error { }) } } fn write_char (& mut self , c : char) -> core :: fmt :: Result { let mut iter = self . expect . chars () ; if iter . next () == Some (c) { self . expect = iter . as_slice () ; Ok (()) } else { Err (core :: fmt :: Error { }) } } }
    };
}

impl_130!()