macro_rules! deps {
    () => {
        Reader!();
        UnitRef!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < 'a , R : Reader > Copy for UnitRef < 'a , R > { }
    };
}

impl_274!();