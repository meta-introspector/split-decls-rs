macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < A : Allocator > Extend < Box < str , A > > for alloc_crate :: string :: String { fn extend < I : IntoIterator < Item = Box < str , A > > > (& mut self , iter : I) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
    };
}

impl_73!()