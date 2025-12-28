macro_rules! deps {
    () => {
        PriorityQueue!();
        Item!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < K : Ord , T > FromIterator < (K , T) > for PriorityQueue < K , T > { fn from_iter < I : IntoIterator < Item = (K , T) > > (iter : I) -> Self { let mut q = PriorityQueue (BinaryHeap :: new ()) ; for (k , v) in iter { q . insert (k , v) ; } q } }
    };
}

impl_36!()