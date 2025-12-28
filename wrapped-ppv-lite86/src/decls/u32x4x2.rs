macro_rules! deps {
    () => {
        Vec2!();
        MultiLane!();
        StoreBytes!();
        ArithOps!();
        Machine!();
        BitOps32!();
        Store!();
    };
}

macro_rules! u32x4x2 {
    () => {
        deps!();
        pub trait u32x4x2 < M : Machine > : BitOps32 + Store < vec256_storage > + Vec2 < M :: u32x4 > + MultiLane < [M :: u32x4 ; 2] > + ArithOps + Into < vec256_storage > + StoreBytes { }
    };
}

u32x4x2!();