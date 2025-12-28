macro_rules! deps {
    () => {
        BitOps64!();
        MultiLane!();
        ArithOps!();
        Vec2!();
        Machine!();
        StoreBytes!();
        Store!();
    };
}

macro_rules! u64x2x2 {
    () => {
        deps!();
        pub trait u64x2x2 < M : Machine > : BitOps64 + Store < vec256_storage > + Vec2 < M :: u64x2 > + MultiLane < [M :: u64x2 ; 2] > + ArithOps + StoreBytes + Into < vec256_storage > { }
    };
}

u64x2x2!()