macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Copy for EdgeReference < '_ , E , Ix > { }
    };
}

impl_288!();