macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < Fut : Future + Unpin > FromIterator < Fut > for SelectAll < Fut > { fn from_iter < T : IntoIterator < Item = Fut > > (iter : T) -> Self { select_all (iter) } }
    };
}

impl_236!()