macro_rules! deps {
    () => {
        MultiLane!();
        Words4!();
        BitOps64!();
        StoreBytes!();
        Vec4!();
        ArithOps!();
        Store!();
        Machine!();
    };
}

macro_rules! u64x4 {
    () => {
        deps!();
        pub trait u64x4 < M : Machine > : BitOps64 + Store < vec256_storage > + Vec4 < u64 > + MultiLane < [u64 ; 4] > + ArithOps + Words4 + StoreBytes + Into < vec256_storage > { }
    };
}

u64x4!();