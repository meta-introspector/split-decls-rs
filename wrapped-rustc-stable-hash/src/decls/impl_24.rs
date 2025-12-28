macro_rules! deps {
    () => {
        State!();
        Sip13Rounds!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Sip13Rounds { # [inline] fn c_rounds (state : & mut State) { compress ! (state) ; } # [inline] fn d_rounds (state : & mut State) { compress ! (state) ; compress ! (state) ; compress ! (state) ; } }
    };
}

impl_24!()