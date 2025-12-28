macro_rules! deps {
    () => {
        Zero!();
        NotZero!();
    };
}

macro_rules! impl_974 {
    () => {
        deps!();
        impl < T : Zero > From < NotZero < T > > for Option < T > { fn from (not_zero : NotZero < T >) -> Self { if ! not_zero . is_null () { Some (not_zero . 0) } else { None } } }
    };
}

impl_974!();