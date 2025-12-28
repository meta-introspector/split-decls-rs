macro_rules! deps {
    () => {
        HashMap!();
        HashSet!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl < T , S , A > From < HashMap < T , () , S , A > > for HashSet < T , S , A > where A : Allocator , { fn from (map : HashMap < T , () , S , A >) -> Self { Self { map } } }
    };
}

impl_399!()