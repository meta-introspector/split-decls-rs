macro_rules! deps {
    () => {
        Words4!();
        Machine!();
        MultiLane!();
        StoreBytes!();
        BitOps64!();
        Vec4!();
        Store!();
        ArithOps!();
    };
}

macro_rules! u64x4 {
    () => {
        deps!();
        pub trait u64x4 < M : Machine > : BitOps64 + Store < vec256_storage > + Vec4 < u64 > + MultiLane < [u64 ; 4] > + ArithOps + Words4 + StoreBytes + Into < vec256_storage > { }
    };
}

u64x4!()