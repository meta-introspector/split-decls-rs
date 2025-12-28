macro_rules! deps {
    () => {
        Machine!();
        MultiLane!();
        ArithOps!();
        BitOps64!();
        Vec2!();
        Store!();
    };
}

macro_rules! u64x2 {
    () => {
        deps!();
        pub trait u64x2 < M : Machine > : BitOps64 + Store < vec128_storage > + ArithOps + Vec2 < u64 > + MultiLane < [u64 ; 2] > + Into < vec128_storage > { }
    };
}

u64x2!();