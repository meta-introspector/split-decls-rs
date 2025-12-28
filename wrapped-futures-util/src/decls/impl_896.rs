macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_896 {
    () => {
        deps!();
        impl < St : Stream + Unpin > Extend < St > for SelectAll < St > { fn extend < T : IntoIterator < Item = St > > (& mut self , iter : T) { for st in iter { self . push (st) } } }
    };
}

impl_896!()