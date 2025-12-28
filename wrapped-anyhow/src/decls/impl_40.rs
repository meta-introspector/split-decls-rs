macro_rules! deps {
    () => {
        Result!();
        Ok!();
        Buf!();
        Error!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Write for Buf { fn write_str (& mut self , s : & str) -> fmt :: Result { if s . bytes () . any (| b | b == b' ' || b == b'\n') { return Err (fmt :: Error) ; } let remaining = self . bytes . len () - self . written ; if s . len () > remaining { return Err (fmt :: Error) ; } unsafe { ptr :: copy_nonoverlapping (s . as_ptr () , self . bytes . as_mut_ptr () . add (self . written) . cast :: < u8 > () , s . len () ,) ; } self . written += s . len () ; Ok (()) } }
    };
}

impl_40!()