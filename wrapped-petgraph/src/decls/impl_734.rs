macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        impl < E , Ix : IndexType > PartialEq for EdgeReference < '_ , E , Ix > where E : PartialEq , { fn eq (& self , rhs : & Self) -> bool { self . index == rhs . index && self . weight == rhs . weight } }
    };
}

impl_734!();