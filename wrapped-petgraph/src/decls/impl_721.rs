macro_rules! deps {
    () => {
        WalkNeighbors!();
        IndexType!();
    };
}

macro_rules! impl_721 {
    () => {
        deps!();
        impl < Ix > Clone for WalkNeighbors < Ix > where Ix : IndexType , { fn clone (& self) -> Self { WalkNeighbors { skip_start : self . skip_start , next : self . next , } } }
    };
}

impl_721!();