macro_rules! deps {
    () => {
        NonZero!();
        Odd!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < T > From < Odd < T > > for NonZero < T > { fn from (odd : Odd < T >) -> NonZero < T > { NonZero (odd . get ()) } }
    };
}

impl_207!()