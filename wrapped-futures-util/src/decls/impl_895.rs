macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_895 {
    () => {
        deps!();
        impl < St : Stream + Unpin > FromIterator < St > for SelectAll < St > { fn from_iter < T : IntoIterator < Item = St > > (iter : T) -> Self { select_all (iter) } }
    };
}

impl_895!()