macro_rules! deps {
    () => {
        EdgeIndex!();
        IndexType!();
        NodeIndex!();
    };
}

macro_rules! impl_655 {
    () => {
        deps!();
        impl < Ix : IndexType > NodeIndex < Ix > { # [inline] pub fn new (x : usize) -> Self { NodeIndex (IndexType :: new (x)) } # [inline] pub fn index (self) -> usize { self . 0 . index () } # [inline] pub fn end () -> Self { NodeIndex (IndexType :: max ()) } fn _into_edge (self) -> EdgeIndex < Ix > { EdgeIndex (self . 0) } }
    };
}

impl_655!()