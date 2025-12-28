macro_rules! deps {
    () => {
        SelectOk!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < Fut : TryFuture + Unpin > FromIterator < Fut > for SelectOk < Fut > { fn from_iter < T : IntoIterator < Item = Fut > > (iter : T) -> Self { select_ok (iter) } }
    };
}

impl_263!()