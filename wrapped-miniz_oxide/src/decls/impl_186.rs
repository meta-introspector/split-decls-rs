macro_rules! deps {
    () => {
        FullReset!();
        ResetPolicy!();
        InflateState!();
        ZeroReset!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl ResetPolicy for FullReset { # [inline] fn reset (& self , state : & mut InflateState) { ZeroReset . reset (state) ; state . data_format = self . 0 ; } }
    };
}

impl_186!();