macro_rules! deps {
    () => {
        PossibleValue!();
        Str!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < S : Into < Str > > From < S > for PossibleValue { fn from (s : S) -> Self { Self :: new (s) } }
    };
}

impl_149!();