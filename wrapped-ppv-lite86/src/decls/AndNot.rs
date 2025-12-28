macro_rules! AndNot {
    () => {
        pub trait AndNot { type Output ; fn andnot (self , rhs : Self) -> Self :: Output ; }
    };
}

AndNot!()