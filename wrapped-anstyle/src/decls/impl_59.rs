macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl core :: fmt :: Display for Style { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { if f . alternate () { self . render_reset () . fmt (f) } else { self . fmt_to (f) } } }
    };
}

impl_59!()