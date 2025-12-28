macro_rules! deps {
    () => {
        Str!();
        PossibleValue!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < S : Into < Str > > From < S > for PossibleValue { fn from (s : S) -> Self { Self :: new (s) } }
    };
}

impl_149!()