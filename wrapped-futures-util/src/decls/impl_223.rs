macro_rules! deps {
    () => {
        JoinAll!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < F : Future > FromIterator < F > for JoinAll < F > { fn from_iter < T : IntoIterator < Item = F > > (iter : T) -> Self { join_all (iter) } }
    };
}

impl_223!()