macro_rules! deps {
    () => {
        EdgeReference!();
        IndexType!();
    };
}

macro_rules! impl_823 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Copy for EdgeReference < '_ , E , Ix > { }
    };
}

impl_823!();