macro_rules! deps {
    () => {
        IsNormalizedSinkStr!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl core :: fmt :: Write for IsNormalizedSinkStr < '_ > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { if core :: ptr :: eq (s . as_ptr () , self . expect . as_ptr ()) { self . expect = & self . expect [s . len () ..] ; Ok (()) } else { Err (core :: fmt :: Error { }) } } fn write_char (& mut self , c : char) -> core :: fmt :: Result { let mut iter = self . expect . chars () ; if iter . next () == Some (c) { self . expect = iter . as_str () ; Ok (()) } else { Err (core :: fmt :: Error { }) } } }
    };
}

impl_136!()