macro_rules! deps {
    () => {
        TryJoinAll!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < F > FromIterator < F > for TryJoinAll < F > where F : TryFuture , { fn from_iter < T : IntoIterator < Item = F > > (iter : T) -> Self { try_join_all (iter) } }
    };
}

impl_250!()