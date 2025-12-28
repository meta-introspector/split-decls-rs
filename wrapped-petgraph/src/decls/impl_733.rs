macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
    };
}

macro_rules! impl_733 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Copy for EdgeReference < '_ , E , Ix > { }
    };
}

impl_733!();