macro_rules! deps {
    () => {
        MultiLane!();
        BitOps128!();
        Swap64!();
        Store!();
        Machine!();
    };
}

macro_rules! u128x1 {
    () => {
        deps!();
        pub trait u128x1 < M : Machine > : BitOps128 + Store < vec128_storage > + Swap64 + MultiLane < [u128 ; 1] > + Into < vec128_storage > { }
    };
}

u128x1!();