macro_rules! deps {
    () => {
        MinReset!();
        TINFLStatus!();
        ResetPolicy!();
        InflateState!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl ResetPolicy for MinReset { fn reset (& self , state : & mut InflateState) { state . decompressor () . init () ; state . dict_ofs = 0 ; state . dict_avail = 0 ; state . first_call = true ; state . has_flushed = false ; state . last_status = TINFLStatus :: NeedsMoreInput ; } }
    };
}

impl_182!();