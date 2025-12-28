macro_rules! deps {
    () => {
        EdgeReference!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl < E , Ty , Ix : Copy > Copy for EdgeReference < '_ , E , Ty , Ix > { }
    };
}

impl_525!()