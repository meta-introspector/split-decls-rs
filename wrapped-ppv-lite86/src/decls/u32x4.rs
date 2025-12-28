macro_rules! deps {
    () => {
        Store!();
        Vec4!();
        LaneWords4!();
        StoreBytes!();
        MultiLane!();
        Machine!();
        ArithOps!();
        Words4!();
        BitOps32!();
    };
}

macro_rules! u32x4 {
    () => {
        deps!();
        pub trait u32x4 < M : Machine > : BitOps32 + Store < vec128_storage > + ArithOps + Vec4 < u32 > + Words4 + LaneWords4 + StoreBytes + MultiLane < [u32 ; 4] > + Into < vec128_storage > { }
    };
}

u32x4!()