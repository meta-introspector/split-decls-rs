macro_rules! deps {
    () => {
        ResetPolicy!();
        ZeroReset!();
        MinReset!();
        InflateState!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl ResetPolicy for ZeroReset { # [inline] fn reset (& self , state : & mut InflateState) { MinReset . reset (state) ; state . dict = [0 ; TINFL_LZ_DICT_SIZE] ; } }
    };
}

impl_184!();