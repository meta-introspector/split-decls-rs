macro_rules! deps {
    () => {
        Machine!();
        BitOps128!();
        Swap64!();
        Vec2!();
        MultiLane!();
        Store!();
    };
}

macro_rules! u128x2 {
    () => {
        deps!();
        pub trait u128x2 < M : Machine > : BitOps128 + Store < vec256_storage > + Vec2 < M :: u128x1 > + MultiLane < [M :: u128x1 ; 2] > + Swap64 + Into < vec256_storage > { }
    };
}

u128x2!()