macro_rules! deps {
    () => {
        Machine!();
        BitOps64!();
        Vec2!();
        ArithOps!();
        Store!();
        MultiLane!();
        StoreBytes!();
    };
}

macro_rules! u64x2x2 {
    () => {
        deps!();
        pub trait u64x2x2 < M : Machine > : BitOps64 + Store < vec256_storage > + Vec2 < M :: u64x2 > + MultiLane < [M :: u64x2 ; 2] > + ArithOps + StoreBytes + Into < vec256_storage > { }
    };
}

u64x2x2!();