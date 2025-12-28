macro_rules! deps {
    () => {
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        unsafe impl < Ix : IndexType > IndexType for NodeIndex < Ix > { fn index (& self) -> usize { self . 0 . index () } fn new (x : usize) -> Self { NodeIndex :: new (x) } fn max () -> Self { NodeIndex (< Ix as IndexType > :: max ()) } }
    };
}

impl_656!()