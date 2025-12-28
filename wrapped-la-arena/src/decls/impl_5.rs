macro_rules! deps {
    () => {
        Idx!();
        ArenaMap!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T , V > Extend < (Idx < V > , T) > for ArenaMap < Idx < V > , T > { fn extend < I : IntoIterator < Item = (Idx < V > , T) > > (& mut self , iter : I) { iter . into_iter () . for_each (move | (k , v) | { self . insert (k , v) ; }) ; } }
    };
}

impl_5!()