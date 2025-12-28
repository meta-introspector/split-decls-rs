macro_rules! deps {
    () => {
        VisitMap!();
        IndexType!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < Ix > VisitMap < Ix > for FixedBitSet where Ix : IndexType , { fn visit (& mut self , x : Ix) -> bool { ! self . put (x . index ()) } fn is_visited (& self , x : & Ix) -> bool { self . contains (x . index ()) } fn unvisit (& mut self , x : Ix) -> bool { if self . is_visited (& x) { self . toggle (x . index ()) ; return true ; } false } }
    };
}

impl_102!();